use core::array;

use alloc::string::ToString;

use crate::{
    display::{
        color::Color256,
        graphics::{Rectangle, WIDTH},
    },
    entities::{button::Button, text::Text},
    game::{Entity, Event, Shareable},
};

pub struct DifficultySelector {
    difficulties: [Button<Text>; Self::NUM_DIFFICULTIES],
}

impl DifficultySelector {
    const NUM_DIFFICULTIES: usize = 7;

    pub fn new() -> Self {
        const Y: usize = 16;
        const BUTTON_SIZE: usize = 16;
        const PADDING: usize = 4;
        let start_x =
            (WIDTH - Self::NUM_DIFFICULTIES * BUTTON_SIZE - (Self::NUM_DIFFICULTIES - 1) * PADDING)
                / 2;

        let difficulties = array::from_fn(|i| {
            let text = (i + 1).to_string();
            let rect = Rectangle {
                x: start_x + i * BUTTON_SIZE + i * PADDING,
                y: Y,
                width: BUTTON_SIZE,
                height: BUTTON_SIZE,
            };
            Button::with_text(rect, text, Event::SetEngineDepth(i as u8 + 1))
        });

        Self { difficulties }
    }
}

impl Entity for DifficultySelector {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        for (i, button) in self.difficulties.iter_mut().enumerate() {
            button.handle_event(event, shared);
            if (i as u8 + 1) == shared.engine_depth {
                button.set_color(Color256::LIGHT_BLUE);
            } else {
                button.set_color(Color256::WHITE);
            }
        }
    }

    fn draw(&self, shared: &Shareable) {
        for button in self.difficulties.iter() {
            button.draw(shared)
        }
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
