use std::collections::HashMap;

use crate::color::{GbaColor, TrueColor};

pub struct Pallete {
    max_count: usize,
    next_free: usize,
    index_by_color: HashMap<GbaColor, usize>,
    color_by_index: HashMap<usize, GbaColor>,
}

impl Pallete {
    pub fn new(max_count: usize) -> Pallete {
        Pallete {
            max_count,
            next_free: 0,
            index_by_color: HashMap::new(),
            color_by_index: HashMap::new(),
        }
    }

    pub fn lookup_or_insert(&mut self, color: TrueColor) -> Option<usize> {
        let color: GbaColor = color.into();
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

    pub fn serialize(&self) -> Vec<u8> {
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

#[test]
fn test_pal() {
    use crate::color::rgba;
    let mut pal = Pallete::new(3);

    let color1 = rgba(255, 0, 0, 255);
    let color2 = rgba(255, 0, 255, 255);
    let color3 = rgba(255, 255, 0, 255);
    let color4 = rgba(255, 255, 255, 255);

    assert_eq!(pal.lookup_or_insert(color1), Some(0));
    assert_eq!(pal.lookup_or_insert(color2), Some(1));
    assert_eq!(pal.lookup_or_insert(color1), Some(0));
    assert_eq!(pal.lookup_or_insert(color3), Some(2));
    assert_eq!(pal.lookup_or_insert(color4), None);
}
