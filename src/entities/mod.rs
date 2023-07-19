

use crate::{
    game::{Event},
};




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
