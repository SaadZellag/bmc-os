
use vga::drawing::Point;

use crate::{display::color::Color256};

// Assumes that the image is from RAW GIMP data
// Just a bunch of RGBA pixels

#[macro_export]
macro_rules! load_sprite {
    ($path:expr, $width:expr) => {{
        let data = include_bytes!($path);
        assert!(data.len() % (($width) * 4) == 0);
        $crate::display::sprite::_new_sprite(data, $width)
    }};
}

#[macro_export]
macro_rules! load_sprite_block {
    ($path:expr, $e_width:expr, $e_height:expr, $elements_per_row:expr) => {{
        const DATA: &'static [u8] = include_bytes!($path);
        assert!(DATA.len() % (($e_width) * ($elements_per_row) * 4) == 0);

        let mut new_data: alloc::vec::Vec<u8> = alloc::vec![0; DATA.len()];
        let mut i = 0;

        while i < DATA.len() {
            let pixel = i / 4;
            let global_char = pixel / ($e_height * $e_width);

            let global_x = (global_char % $elements_per_row) * $e_width;
            let global_y = global_char / ($elements_per_row) * $e_height;

            let local_x = pixel % $e_width;
            let local_y = (pixel % ($e_width * $e_height)) / $e_width;

            let final_x = global_x + local_x;
            let final_y = global_y + local_y;


            let index = 4 * (final_y * $e_width * $elements_per_row + final_x);

            new_data[i] = DATA[index];
            new_data[i+1] = DATA[index+1];
            new_data[i+2] = DATA[index+2];
            new_data[i+3] = DATA[index+3];

            i += 4;
        }

        $crate::display::sprite::SpriteBlock::new(new_data.leak(), $e_width, $e_height)
    }};
}

pub const fn _new_sprite(data: &'static [u8], width: usize) -> Sprite {
    Sprite { data, width }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    data: &'static [u8],
    width: usize,
}

#[derive(Debug, Clone)]
pub struct SpriteBlock {
    data: &'static [u8],
    element_width: usize,
    element_height: usize,
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

    pub fn width(&self) -> usize {
        self.width
    }
}

impl SpriteBlock {
    pub const fn new(data: &'static [u8], element_width: usize, element_height: usize) -> Self {
        Self {
            data,
            element_width,
            element_height,
        }
    }

    pub fn index(&self, index: usize) -> Sprite {
        let element_size = self.element_height * self.element_width * 4;
        let num_elements = self.data.len() / element_size;

        if index >= num_elements {
            panic!("Invalid index {}, size is {}", index, num_elements);
        }

        let slice = &self.data[(index * element_size)..((index + 1) * element_size)];

        Sprite {
            data: slice,
            width: self.element_width,
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
