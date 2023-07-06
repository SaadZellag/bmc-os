use bresenham::Bresenham;
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

use crate::{
    display::{
        color::Color256,
        ensure_graphics_mode, get_current_graphics_color, get_current_text_color,
        sprite::{PixelInfo, Sprite, SpriteBlock},
        CURRENT_GRAPHICS_COLOR, DRAWER,
    },
    load_sprite_block, println,
};

pub const CHAR_WIDTH: usize = 8;
pub const CHAR_HEIGHT: usize = 8;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
const SIZE: usize = WIDTH * HEIGHT;

static mut BUFFER: [Color256; SIZE] = [Color256::BLACK; SIZE];

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

// https://lpc.opengameart.org/content/8x8-ascii-bitmap-font-with-c-source
lazy_static! {
    static ref TEXT: SpriteBlock =
        load_sprite_block!("../../sprites/Text.data", CHAR_WIDTH, CHAR_HEIGHT, 16);
}

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

// https://stackoverflow.com/questions/22521982/check-if-point-is-inside-a-polygon
pub fn contains_point<const N: usize, S>(shape: &S, (x, y): Point<usize>) -> bool
where
    S: Shape<Output = [Point<usize>; N]>,
{
    let points = shape.points();

    let x = x as f32;
    let y = y as f32;

    let mut inside = false;
    let mut i = 0;
    let mut j = N - 1;

    while i < N {
        let (xi, yi) = points[i];
        let (xj, yj) = points[j];

        let xi = xi as f32;
        let yi = yi as f32;
        let xj = xj as f32;
        let yj = yj as f32;
        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }

        j = i;
        i += 1;
    }

    inside
}

#[macro_export]
macro_rules! set_pixel {
    ($x:expr, $y:expr) => {{
        $crate::display::graphics::_set_pixel_with_lock($x, $y);
    }};

    ($x:expr, $y:expr, $color:expr) => {{
        $crate::display::graphics::_set_pixel($x, $y, $color);
    }};
}

pub fn draw_sprite(sprite: &Sprite, x: usize, y: usize) {
    let itt = sprite.to_absolute_points(x, y);
    draw_itt(itt);
}

pub fn draw_itt(itt: impl Iterator<Item = PixelInfo>) {
    for pixel in itt {
        let (x, y) = pixel.pos;
        let current_color = get_pixel(x, y);
        let new_color =
            current_color.apply_alpha(255 - pixel.alpha) + pixel.color.apply_alpha(pixel.alpha);
        set_pixel!(x, y, new_color);
    }
}

pub fn draw_line(start: Point<usize>, end: Point<usize>) {
    let color = *CURRENT_GRAPHICS_COLOR.lock();
    let start = (start.0 as isize, start.1 as isize);
    let end = (end.0 as isize, end.1 as isize);
    for (x, y) in Bresenham::new(start, end) {
        set_pixel!(x as usize, y as usize, color)
    }
}

pub fn draw_text(text: impl Iterator<Item = u8>, x: usize, y: usize) {
    let current_color = get_current_graphics_color();

    for (i, c) in text.enumerate() {
        let sprite = TEXT.index(c as usize);

        let x = x + i * sprite.width();

        let itt = sprite.to_absolute_points(x, y);

        // Overriding text color
        draw_itt(itt.map(|mut pixel| {
            pixel.color = current_color;
            pixel
        }));
    }
}

pub fn clear_buffer() {
    unsafe {
        for p in BUFFER.iter_mut() {
            *p = Color256::BLACK;
        }
    }
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
                frame_buffer.add(j).write_volatile(color.as_u8());
            }
        }
    }
}

pub fn _set_pixel_with_lock(x: usize, y: usize) {
    let color = *CURRENT_GRAPHICS_COLOR.lock();
    _set_pixel(x, y, color);
}

pub fn _set_pixel(x: usize, y: usize, color: Color256) {
    unsafe {
        BUFFER[y * WIDTH + x] = color;
    }
}

fn get_pixel(x: usize, y: usize) -> Color256 {
    unsafe { BUFFER[y * WIDTH + x] }
}

#[test_case]
fn test_in_point_rectangle() {
    let rect = Rectangle {
        x: 1,
        y: 2,
        width: 3,
        height: 4,
    };

    let in_point = (3, 5);
    let out_point = (6, 5);

    assert!(contains_point(&rect, in_point));
    assert!(!contains_point(&rect, out_point));
}

#[test_case]
fn test_in_point_triangle() {
    let triangle = Triangle {
        points: [(0, 0), (5, 0), (2, 5)],
    };

    let in_point = (2, 1);
    let out_point = (0, 5);

    assert!(contains_point(&triangle, in_point));
    assert!(!contains_point(&triangle, out_point));
}
