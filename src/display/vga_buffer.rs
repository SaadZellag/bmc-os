use crate::display::{Color, ColorCode};
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    index: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_str(&mut self, content: &str) {
        for c in content.bytes() {
            self.write_byte(c);
        }
    }

    pub fn write_byte(&mut self, c: u8) {
        if self.index >= BUFFER_HEIGHT * BUFFER_WIDTH {
            self.shift_up();
        }
        match c {
            b'\n' => self.new_line(),
            _ => {
                let x = self.index % BUFFER_WIDTH;
                let y = self.index / BUFFER_WIDTH;
                self.buffer.chars[y][x] = ScreenChar {
                    ascii_character: c,
                    color_code: self.color_code,
                };
                self.index += 1;
            }
        }
    }

    pub fn set_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }

    pub fn get_color(&self) -> ColorCode {
        self.color_code
    }

    fn new_line(&mut self) {
        self.index = (self.index / BUFFER_WIDTH + 1) * BUFFER_WIDTH
    }

    fn shift_up(&mut self) {
        for i in 0..(BUFFER_HEIGHT - 1) {
            for j in 0..BUFFER_WIDTH {
                self.buffer.chars[i][j] = self.buffer.chars[i + 1][j];
            }
        }
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][i] = ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode(0),
            };
        }
        self.index -= BUFFER_WIDTH;
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        index: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
