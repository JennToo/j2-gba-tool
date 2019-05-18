use image::RgbaImage;

use crate::fatal;
use crate::pal::Palette;

pub fn convert_image(img: RgbaImage) -> (Vec<u8>, Vec<u8>) {
    const TILE_SIZE: u32 = 8;
    let actual_dims = img.dimensions();

    if actual_dims.0 % TILE_SIZE != 0 || actual_dims.1 % TILE_SIZE != 0 {
        fatal!(
            "Dimensions {:?} are not a multiple of tile size {}",
            actual_dims,
            TILE_SIZE
        );
    }
    let mut palette = Palette::new(256);
    let mut out_pixels = vec![];

    let tile_dims = (actual_dims.0 / 8, actual_dims.1 / 8);
    for tile_y in 0..tile_dims.1 {
        for tile_x in 0..tile_dims.0 {
            for sub_y in 0..TILE_SIZE {
                for sub_x in 0..TILE_SIZE {
                    let src_x = sub_x + tile_x * TILE_SIZE;
                    let src_y = sub_y + tile_y * TILE_SIZE;

                    let pixel = img.get_pixel(src_x, src_y);
                    let value = palette.lookup_or_insert(*pixel);
                    if value.is_none() {
                        fatal!("Image has more colors than palette supports",);
                    }
                    out_pixels.push(value.unwrap() as u8);
                }
            }
        }
    }

    (out_pixels, palette.serialize())
}
