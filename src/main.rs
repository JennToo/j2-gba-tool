use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;

use clap;
use colored;
use image;

use image::{Pixel, RgbaImage};

macro_rules! fatal {
    ($fmt_string:expr, $( $arg:expr ),*) => {
        use colored::*;
        eprint!("{} ", "Error:".bold().red());
        eprintln!($fmt_string, $( $arg ),*);
        std::process::exit(0);
    }
}

fn main() {
    let matches = clap::App::new("j2-gba-tool")
        .author("Jennifer Wilcox <jennifer@nitori.org>")
        .about("A GBA ROM multi-tool")
        .subcommand(
            clap::SubCommand::with_name("gfx-convert")
                .about("Convert typical image formats to GBA formats")
                .arg(
                    clap::Arg::with_name("format")
                        .value_name("bg256c1p")
                        .required(true)
                        .help("Output format"),
                )
                .arg(
                    clap::Arg::with_name("input-file")
                        .required(true)
                        .help("Source image in a typical format (PNG, BMP, etc.)"),
                )
                .arg(
                    clap::Arg::with_name("pixel-output-file")
                        .required(true)
                        .help("Binary output of the pixel data"),
                )
                .arg(
                    clap::Arg::with_name("pallete-output-file")
                        .required(false)
                        .help("Binary output of the pallete data (if relevant)"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("gfx-convert") {
        let input_path = Path::new(matches.value_of("input-file").unwrap());
        let input_img = image::open(&input_path).expect("Failed to open input image");

        let format = matches.value_of("format").unwrap();

        match format {
            "bg256c1p" => {
                let (pixels, pal) = convert_image(input_img.to_rgba());
                write_bytes_to_file(matches.value_of("pixel-output-file").unwrap(), pixels.as_slice());
                write_bytes_to_file(matches.value_of("pallete-output-file").unwrap(), pal.as_slice());
            }
            _ => {
                fatal!("Unsupported format {}", format);
            }
        }
    }
}

fn write_bytes_to_file(path: &str, data: &[u8]) {
    let mut file =
        std::fs::File::create(path)
        .expect("Failed to open output file");
    file.write_all(data)
        .expect("Failed to write output file");
}

fn convert_image(img: RgbaImage) -> (Vec<u8>, Vec<u8>) {
    const TILE_SIZE: u32 = 8;
    let actual_dims = img.dimensions();

    if actual_dims.0 % TILE_SIZE != 0 || actual_dims.1 % TILE_SIZE != 0 {
        fatal!(
            "Dimensions {:?} are not a multiple of tile size {}",
            actual_dims,
            TILE_SIZE
        );
    }
    let mut pallete = Pallete::new(256);
    let mut out_pixels = vec![];

    let tile_dims = (actual_dims.0 / 8, actual_dims.1 / 8);
    for tile_y in 0..tile_dims.1 {
        for tile_x in 0..tile_dims.0 {
            for sub_y in 0..TILE_SIZE {
                for sub_x in 0..TILE_SIZE {
                    let src_x = sub_x + tile_x * TILE_SIZE;
                    let src_y = sub_y + tile_y * TILE_SIZE;

                    let pixel = img.get_pixel(src_x, src_y);
                    let value = pallete.lookup_or_insert(*pixel);
                    if value.is_none() {
                        fatal!("Image has more colors than pallete supports",);
                    }
                    out_pixels.push(value.unwrap() as u8);
                }
            }
        }
    }

    (out_pixels, pallete.serialize())
}

type Color = image::Rgba<u8>;
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct GbaColor(u16);

struct Pallete {
    max_count: usize,
    next_free: usize,
    index_by_color: HashMap<GbaColor, usize>,
    color_by_index: HashMap<usize, GbaColor>,
}

impl Pallete {
    fn new(max_count: usize) -> Pallete {
        Pallete {
            max_count,
            next_free: 0,
            index_by_color: HashMap::new(),
            color_by_index: HashMap::new(),
        }
    }

    fn lookup_or_insert(&mut self, color: Color) -> Option<usize> {
        let color = GbaColor::from_color(color);
        if let Some(idx) = self.index_by_color.get(&color) {
            Some(*idx)
        } else if self.next_free == self.max_count {
            None
        } else {
            let idx = self.next_free;
            self.next_free += 1;
            self.index_by_color.insert(color, idx);
            self.color_by_index.insert(idx, color);
            Some(idx)
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut data = vec![];
        for i in 0..self.max_count {
            if let Some(color) = self.color_by_index.get(&i) {
                data.push(color.0 as u8);
                data.push((color.0 >> 8) as u8);
            } else {
                data.push(0);
                data.push(0);
            }
        }

        data
    }
}

fn map_channel(chan: u8) -> u16 {
    const COLOR_MASK: u8 = 0b1_1111;
    ((chan >> 3) & COLOR_MASK) as u16
}

impl GbaColor {
    fn from_color(color: Color) -> GbaColor {
        GbaColor(
            map_channel(color.channels()[0])
                | (map_channel(color.channels()[1]) << 5)
                | (map_channel(color.channels()[2]) << 10),
        )
    }
}
