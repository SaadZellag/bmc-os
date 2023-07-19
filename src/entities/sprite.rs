use crate::{
    display::{graphics::draw_sprite, sprite::Sprite},
    game::{Entity, Event, Shareable},
};

pub struct SpriteEntity {
    x: usize,
    y: usize,
    sprite: &'static Sprite,
}

impl SpriteEntity {
    pub fn new(x: usize, y: usize, sprite: &'static Sprite) -> Self {
        Self { x, y, sprite }
    }
}

impl Entity for SpriteEntity {
    fn handle_event(&mut self, _: &Event, _: &Shareable) {}

    fn draw(&self, _: &Shareable) {
        draw_sprite(self.sprite, self.x, self.y)
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
