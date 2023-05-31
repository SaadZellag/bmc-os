use vga::drawing::Point;

use crate::{display::color::Color256, print, println};

// Assumes that the image is from RAW GIMP data
// Just a bunch of RGBA pixels

#[macro_export]
macro_rules! load_sprite {
    ($path:expr, $width:expr) => {{
        let data = include_bytes!($path);
        assert!(data.len() % ($width * 4) == 0);
        $crate::display::sprite::_new_sprite(data, $width)
    }};
}

pub fn _new_sprite(data: &'static [u8], width: usize) -> Sprite {
    Sprite { data, width }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    data: &'static [u8],
    width: usize,
}

#[derive(Debug, Clone)]
pub struct SpriteIterator<'a> {
    x: usize,
    y: usize,
    sprite: &'a Sprite,
    index: usize,
}

#[derive(Debug, Clone)]
pub struct PixelInfo {
    pub pos: Point<usize>,
    pub color: Color256,
    pub alpha: u8,
}

impl Sprite {
    pub fn new(data: &'static [u8], width: usize) -> Self {
        assert_eq!(data.len() % width, 0);
        Self { data, width }
    }

    pub fn to_absolute_points<'a>(&'a self, x: usize, y: usize) -> SpriteIterator<'a> {
        SpriteIterator {
            x,
            y,
            sprite: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for SpriteIterator<'a> {
    type Item = PixelInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let sprite = self.sprite;
        if index >= sprite.data.len() {
            return None;
        }
        let r = sprite.data[index];
        let g = sprite.data[index + 1];
        let b = sprite.data[index + 2];
        let a = sprite.data[index + 3];

        let absolute_x = self.x + (index / 4) % self.sprite.width;
        let absolute_y = self.y + (index / 4) / self.sprite.width;
        self.index += 4;

        Some(PixelInfo {
            pos: (absolute_x, absolute_y),
            color: Color256::new(r, g, b),
            alpha: a,
        })
    }
}
