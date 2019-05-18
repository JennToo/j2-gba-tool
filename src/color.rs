use image::Pixel;

pub type TrueColor = image::Rgba<u8>;
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct GbaColor(pub u16);

fn map_channel(chan: u8) -> u16 {
    const COLOR_MASK: u8 = 0b1_1111;
    ((chan >> 3) & COLOR_MASK) as u16
}

#[cfg(test)]
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> TrueColor {
    TrueColor { data: [r, g, b, a] }
}

impl From<TrueColor> for GbaColor {
    fn from(color: TrueColor) -> GbaColor {
        GbaColor(
            map_channel(color.channels()[0])
                | (map_channel(color.channels()[1]) << 5)
                | (map_channel(color.channels()[2]) << 10),
        )
    }
}

#[test]
fn test_gba_color() {
    assert_eq!(
        GbaColor::from(rgba(0b00000000, 0b00000000, 0b00000000, 255)),
        GbaColor(0b0_00000_00000_00000)
    );
    assert_eq!(
        GbaColor::from(rgba(0b00000000, 0b00000000, 0b10000000, 255)),
        GbaColor(0b0_10000_00000_00000)
    );
}
