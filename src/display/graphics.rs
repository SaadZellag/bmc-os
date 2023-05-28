use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    colors::PALETTE_SIZE,
    drawing::Point,
    registers::PlaneMask,
    vga::VGA,
    writers::{Graphics320x240x256, GraphicsWriter},
};
use x86_64::instructions::interrupts;

use crate::display::{color::Color256, ensure_graphics_mode, CURRENT_GRAPHICS_COLOR, DRAWER};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const SIZE: usize = WIDTH * HEIGHT;

static mut BUFFER: [u8; SIZE] = [0; SIZE];

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
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Triangle {
    pub points: [Point<usize>; 3],
}

impl Shape for Rectangle {
    type Output = [Point<usize>; 4];
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
    type Output = [Point<usize>; 3];

    fn points(&self) -> Self::Output {
        self.points.clone()
    }
}

pub fn draw_shape<const N: usize, S>(shape: &S)
where
    S: Shape<Output = [Point<usize>; N]>,
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

pub fn set_pixel(x: usize, y: usize, color: Color256) {
    unsafe {
        BUFFER[y * WIDTH + x] = color.as_u8();
    }
}

pub fn draw_line(start: Point<usize>, end: Point<usize>) {
    todo!()
}

pub fn flush_buffer() {
    const PLANE_MASKS: [PlaneMask; 4] = [
        PlaneMask::PLANE0,
        PlaneMask::PLANE1,
        PlaneMask::PLANE2,
        PlaneMask::PLANE3,
    ];

    ensure_graphics_mode();

    let frame_buffer = { DRAWER.lock().get_frame_buffer() };
    let mut vga = VGA.lock();

    for (i, &plane) in PLANE_MASKS.iter().enumerate() {
        vga.sequencer_registers.set_plane_mask(plane);
        for j in 0..(SIZE / 4) {
            unsafe {
                let color = BUFFER[j * 4 + i];
                frame_buffer.add(j).write_volatile(color);
            }
        }
    }
}
