use alloc::{format, string::String};
use engine::Eval;

use crate::{
    display::{color::Color256, graphics::Rectangle},
    entities::{
        chessboard::{BOARD_X, BOARD_Y, BORDER_SIZE, SQUARE_SIZE},
        text::Text,
    },
    game::{Entity, Event, Shareable},
};

pub const ENGINE_EVAL_X: usize = BOARD_X + 8 * SQUARE_SIZE + BORDER_SIZE + 5;
pub const ENGINE_EVAL_Y: usize = BOARD_Y;

pub struct EngineEval {
    text: Text,
    curr_eval: Eval,
}

impl EngineEval {
    pub fn new() -> Self {
        let mut s = Self {
            text: Text::new(
                Rectangle {
                    x: ENGINE_EVAL_X,
                    y: ENGINE_EVAL_Y,
                    width: 6 * 8,
                    height: 16,
                },
                Self::eval_to_string(Eval::NEUTRAL),
            ),
            curr_eval: Eval::NEUTRAL,
        };
        s.calculate_new_color();
        s
    }

    pub fn eval_to_string(eval: Eval) -> String {
        match eval {
            Eval::MateIn(x) => format!("M{}", x),
            Eval::MatedIn(x) => format!("M-{}", x),
            Eval::CentiPawn(x) => format!("{}", x),
        }
    }

    fn calculate_new_color(&mut self) {
        // Calculating color to show
        // red is losing, green is winning, yellow is neutral

        let val = self.curr_eval.value();

        let computed = sigmoid(val);

        let r = (255 - computed) as u8;
        let g = computed as u8;
        let b = 0;

        let color = Color256::new(r, g, b);

        self.text.set_color(color)
    }
}

impl Entity for EngineEval {
    fn handle_event(&mut self, _: &Event, shared: &Shareable) {
        if shared.engine_eval == self.curr_eval {
            return;
        }

        self.curr_eval = shared.engine_eval;
        self.text.set_text(Self::eval_to_string(self.curr_eval));

        self.calculate_new_color();
    }

    fn draw(&self, shared: &Shareable) {
        self.text.draw(shared);
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

fn sigmoid(val: i32) -> i32 {
    let computed = ((val / 20).abs().min(127) - 127).pow(2) / 128;

    if val > 0 {
        255 - computed
    } else {
        computed
    }
}

#[test_case]
fn test_sigmoid() {
    assert_eq!(sigmoid(0), 126);

    assert_eq!(sigmoid(100000000), 255);
    assert_eq!(sigmoid(-100000000), 0);
}
