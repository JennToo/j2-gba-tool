use std::path::Path;

use clap;
use image;

fn main() {
    let matches = clap::App::new("j2-gba-tool")
        .author("Jennifer Wilcox <jennifer@nitori.org>")
        .about("A GBA ROM multi-tool")
        .subcommand(
            clap::SubCommand::with_name("gfx-convert")
                .about("Convert typical image formats to GBA formats")
                .arg(
                    clap::Arg::with_name("format")
                        .value_name("bg256c1p|bg16c16p")
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
    }
}
