use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    colors::{TextModeColor},
    vga::VGA,
    writers::{Graphics320x240x256, GraphicsWriter, Text80x25, TextWriter},
};
use x86_64::instructions::interrupts;

use crate::display::{color::Color256, graphics::PALETTE, text::WRITER};

pub mod color;
pub mod graphics;
pub mod sprite;
mod text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Text,
    Graphics,
}

lazy_static! {
    static ref CURRENT_MODE: Mutex<Mode> = Mutex::new(Mode::Text);
    static ref TEXT: Mutex<Text80x25> = Mutex::new(Text80x25::new());
    static ref DRAWER: Mutex<Graphics320x240x256> = Mutex::new(Graphics320x240x256::new());
    static ref CURRENT_GRAPHICS_COLOR: Mutex<Color256> = Mutex::new(Color256::WHITE);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::display::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! set_text_color {
    ($foreground:expr, $background:expr) => {{
        use vga::colors::TextModeColor;
        $crate::display::_set_text_color(TextModeColor::new($foreground, $background));
    }};
    ($color_code: expr) => {{
        $crate::display::_set_text_color($color_code);
    }};
}

pub fn set_graphics_color(color: Color256) {
    *CURRENT_GRAPHICS_COLOR.lock() = color;
}

pub fn get_current_graphics_color() -> Color256 {
    *CURRENT_GRAPHICS_COLOR.lock()
}

pub fn get_current_text_color() -> TextModeColor {
    WRITER.lock().get_color()
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        ensure_text_mode();
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn _set_text_color(color: TextModeColor) {
    WRITER.lock().set_color(color);
}

fn ensure_text_mode() {
    let mut current_mode = CURRENT_MODE.lock();
    if *current_mode != Mode::Text {
        let text = TEXT.lock();
        text.set_mode();
        text.clear_screen();
        *current_mode = Mode::Text;
    }
}

pub fn ensure_graphics_mode() {
    let mut current_mode: spin::MutexGuard<Mode> = CURRENT_MODE.lock();
    if *current_mode != Mode::Graphics {
        let drawer = DRAWER.lock();
        drawer.set_mode();
        {
            // In a block to drop it directly
            let mut vga = VGA.lock();
            vga.color_palette_registers.load_palette(&PALETTE);
        }
        drawer.clear_screen(Color256::BLACK.as_u8());
        *current_mode = Mode::Graphics;
    }
}
