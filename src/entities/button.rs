use alloc::string::String;

use crate::{
    display::{
        color::Color256,
        graphics::{contains_point, Rectangle},
        sprite::Sprite,
    },
    entities::{is_mouse_click, sprite::SpriteEntity, text::Text},
    events::add_event,
    game::{Entity, Event, Shareable},
};

pub struct Button<E: Entity> {
    rect: Rectangle,
    entity: E,
    on_click: Event,
}

impl<E: Entity> Button<E> {
    pub fn new(rect: Rectangle, entity: E, on_click: Event) -> Self {
        Self {
            rect,
            entity,
            on_click,
        }
    }

    pub fn rect(&self) -> &Rectangle {
        &self.rect
    }
}

impl Button<Text> {
    pub fn with_text<S: Into<String>>(rect: Rectangle, text: S, on_click: Event) -> Self {
        Self {
            rect,
            entity: Text::new(rect, text),
            on_click,
        }
    }

    pub fn set_color(&mut self, color: Color256) {
        self.entity.set_color(color);
    }
}

impl Button<SpriteEntity> {
    pub fn with_sprite(rect: Rectangle, sprite: &'static Sprite, on_click: Event) -> Self {
        Self {
            rect,
            entity: SpriteEntity::new(rect.x, rect.y, sprite),
            on_click,
        }
    }
}

impl<E: Entity> Entity for Button<E> {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let point = (shared.mouse_x as usize, shared.mouse_y as usize);
        if contains_point(&self.rect, point) {
            add_event(self.on_click.clone());
        }
    }

    fn draw(&self, shared: &Shareable) {
        self.entity.draw(shared)
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
