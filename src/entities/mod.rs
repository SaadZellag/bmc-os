use core::{array, f64::consts};

use crate::{
    display::{
        color::Color256,
        graphics::{
            contains_point, draw_shape, draw_sprite, draw_text, Rectangle, CHAR_HEIGHT, CHAR_WIDTH,
            WIDTH,
        },
        set_graphics_color,
        sprite::Sprite,
    },
    events::add_event,
    game::{Entity, Event, Shareable, State},
    load_sprite,
};
use alloc::{
    format,
    string::{String, ToString},
};
use cozy_chess::{Board, BoardBuilder, Color, File, Move, Piece, Rank, Square};
use engine::Eval;

pub mod button;
pub mod chessboard;
pub mod colorselector;
pub mod difficultyselector;
pub mod engineeval;
pub mod enginethinking;
pub mod promotion;
pub mod sprite;
pub mod text;

fn is_mouse_click(event: &Event) -> bool {
    match event {
        Event::MouseInput(input) => input.left_button_down(),
        _ => false,
    }
}
