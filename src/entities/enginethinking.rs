use crate::{
    display::{graphics::draw_sprite, sprite::Sprite},
    entities::{
        chessboard::SQUARE_SIZE,
        engineeval::{ENGINE_EVAL_X, ENGINE_EVAL_Y},
    },
    game::Entity,
    load_sprite,
};

const ENGINE_THINKING: Sprite = load_sprite!("../../sprites/EngineThinking.data", 40);

const ENGINE_THINKING_X: usize = ENGINE_EVAL_X;
const ENGINE_THINKING_Y: usize = ENGINE_EVAL_Y + SQUARE_SIZE * 2;

pub struct EngineThinking;

impl Entity for EngineThinking {
    fn handle_event(&mut self, _event: &crate::game::Event, _shared: &crate::game::Shareable) {}

    fn draw(&self, shared: &crate::game::Shareable) {
        if shared.engine_thinking {
            draw_sprite(&ENGINE_THINKING, ENGINE_THINKING_X, ENGINE_THINKING_Y);
        }
    }

    fn to_delete(&self, _shared: &crate::game::Shareable) -> bool {
        false
    }
}
