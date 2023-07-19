use alloc::string::String;

use crate::{
    display::{
        color::Color256,
        graphics::{draw_shape, draw_text, Rectangle, CHAR_HEIGHT, CHAR_WIDTH},
        set_graphics_color,
    },
    game::{Entity, Event, Shareable},
};

pub struct Text {
    rect: Rectangle,
    text: String,
    color: Color256,
}

impl Text {
    pub fn new<S>(rect: Rectangle, text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            rect,
            text: text.into(),
            color: Color256::WHITE,
        }
    }

    pub fn set_color(&mut self, color: Color256) {
        self.color = color;
    }

    pub fn set_text<S>(&mut self, text: S)
    where
        S: Into<String>,
    {
        self.text = text.into()
    }
}

impl Entity for Text {
    fn handle_event(&mut self, _: &Event, _: &Shareable) {}

    fn draw(&self, _: &Shareable) {
        set_graphics_color(self.color);

        draw_shape(&self.rect);

        let width = self.text.len() * CHAR_WIDTH;

        let x = self.rect.x + (self.rect.width - width) / 2;
        let y = self.rect.y + (self.rect.height - CHAR_HEIGHT) / 2;

        draw_text(self.text.bytes(), x, y);
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
