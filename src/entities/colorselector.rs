use crate::{
    display::{
        color::Color256,
        graphics::{draw_shape, Rectangle, WIDTH},
        set_graphics_color,
        sprite::Sprite,
    },
    entities::{button::Button, sprite::SpriteEntity},
    game::{Entity, Event, Shareable},
    load_sprite,
};

const CHOOSE_WHITE_SPRITE: Sprite = load_sprite!("../../sprites/White.data", 32);
const CHOOSE_BLACK_SPRITE: Sprite = load_sprite!("../../sprites/Black.data", 32);

pub struct ColorSelector {
    white_button: Button<SpriteEntity>,
    black_button: Button<SpriteEntity>,
}

impl ColorSelector {
    pub fn new() -> Self {
        const CHOOSE_WHITE: Rectangle = Rectangle {
            x: (WIDTH - 80) / 2,
            y: 64,
            width: 32,
            height: 32,
        };

        const CHOOSE_BLACK: Rectangle = Rectangle {
            x: (WIDTH + 80) / 2 - 32,
            y: 64,
            width: 32,
            height: 32,
        };

        let white_button = Button::with_sprite(
            CHOOSE_WHITE,
            &CHOOSE_WHITE_SPRITE,
            Event::SetPlayerColor(cozy_chess::Color::White),
        );
        let black_button = Button::with_sprite(
            CHOOSE_BLACK,
            &CHOOSE_BLACK_SPRITE,
            Event::SetPlayerColor(cozy_chess::Color::Black),
        );

        Self {
            white_button,
            black_button,
        }
    }
}

impl Entity for ColorSelector {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        self.white_button.handle_event(event, shared);
        self.black_button.handle_event(event, shared);
    }

    fn draw(&self, shared: &Shareable) {
        self.white_button.draw(shared);
        self.black_button.draw(shared);
        let rect = match shared.user_color {
            cozy_chess::Color::White => self.white_button.rect(),
            cozy_chess::Color::Black => self.black_button.rect(),
        };

        set_graphics_color(Color256::LIGHT_BLUE);
        draw_shape(rect);
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
