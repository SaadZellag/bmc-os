use vga::{colors::PALETTE_SIZE, drawing::Point};

use crate::display::draw_line;

pub const PALETTE: [u8; PALETTE_SIZE] = {
    let mut palette = [0_u8; PALETTE_SIZE];
    let mut i = 0;

    let mut r: u8 = 0;

    while r < 8 {
        let mut g: u8 = 0;
        while g < 8 {
            let mut b: u8 = 0;
            while b < 4 {
                palette[i] = r * 8;
                palette[i + 1] = g * 8;
                palette[i + 2] = b * 16;
                i += 3;
                b += 1;
            }
            g += 1;
        }
        r += 1;
    }

    palette
};

pub trait Shape {
    type Output;
    fn points(&self) -> Self::Output;
}

pub struct Rectangle {
    pub x: isize,
    pub y: isize,
    pub width: isize,
    pub height: isize,
}

pub struct Triangle {
    pub points: [Point<isize>; 3],
}

impl Shape for Rectangle {
    type Output = [Point<isize>; 4];
    fn points(&self) -> Self::Output {
        [
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
            (self.x, self.y + self.height),
        ]
    }
}

impl Shape for Triangle {
    type Output = [Point<isize>; 3];

    fn points(&self) -> Self::Output {
        self.points.clone()
    }
}

pub fn draw_shape<const N: usize, S>(shape: &S)
where
    S: Shape<Output = [Point<isize>; N]>,
{
    let points = shape.points();
    for line in points.windows(2) {
        let from = line[0];
        let to = line[1];

        draw_line(from, to);
    }
    // Don't forget the end point with the start point
    match (points.get(0), points.last()) {
        (Some(&start), Some(&end)) => {
            draw_line(end, start);
        }
        _ => {}
    }
}
