use std::io::prelude::*;
use std::path::Path;

mod color;
mod image;
mod pal;

#[macro_export]
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
                    clap::Arg::with_name("palette-output-file")
                        .required(false)
                        .help("Binary output of the palette data (if relevant)"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("gfx-convert") {
        let input_path = Path::new(matches.value_of("input-file").unwrap());
        let input_img = ::image::open(&input_path).expect("Failed to open input image");

        let format = matches.value_of("format").unwrap();

        match format {
            "bg256c1p" => {
                let (pixels, pal) = image::convert_image(input_img.to_rgba());
                write_bytes_to_file(
                    matches.value_of("pixel-output-file").unwrap(),
                    pixels.as_slice(),
                );
                write_bytes_to_file(
                    matches.value_of("palette-output-file").unwrap(),
                    pal.as_slice(),
                );
            }
            _ => {
                fatal!("Unsupported format {}", format);
            }
        }
    }
}

fn write_bytes_to_file(path: &str, data: &[u8]) {
    let mut file = std::fs::File::create(path).expect("Failed to open output file");
    file.write_all(data).expect("Failed to write output file");
}
