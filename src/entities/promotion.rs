use cozy_chess::{Move, Piece, Square};

use crate::{
    display::{graphics::draw_sprite, sprite::Sprite},
    entities::{
        chessboard::{piece_sprite, BOARD_X, BOARD_Y, SQUARE_SIZE},
        is_mouse_click,
    },
    events::add_event,
    game::{Entity, Event, Shareable},
    load_sprite,
};

const PROMOTION_X: usize = BOARD_X + 2 * SQUARE_SIZE;
const PROMOTION_Y: usize = (BOARD_Y - SQUARE_SIZE) / 2;

const PROMOTION_BACKGROUND: Sprite =
    load_sprite!("../../sprites/PromotionBackground.data", SQUARE_SIZE * 4);

pub struct PromotionDisplayer {
    from: Square,
    to: Square,
    to_delete: bool,
}

impl PromotionDisplayer {
    const PIECES: [cozy_chess::Piece; 4] =
        [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];

    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            to_delete: false,
        }
    }
}

impl Entity for PromotionDisplayer {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let promotion_x = (shared.mouse_x as usize).wrapping_sub(PROMOTION_X) / SQUARE_SIZE;
        let promotion_y = (shared.mouse_y as usize).wrapping_sub(PROMOTION_Y) / SQUARE_SIZE;

        if promotion_x > 3 || promotion_y > 0 {
            return;
        }

        let piece = PromotionDisplayer::PIECES[promotion_x];

        add_event(Event::PlayMove(Move {
            from: self.from,
            to: self.to,
            promotion: Some(piece),
        }));

        self.to_delete = true;
    }

    fn draw(&self, shared: &Shareable) {
        let color = shared.board.side_to_move();

        draw_sprite(&PROMOTION_BACKGROUND, PROMOTION_X, PROMOTION_Y);

        for (i, piece) in PromotionDisplayer::PIECES.into_iter().enumerate() {
            let sprite = piece_sprite(piece, color);
            draw_sprite(sprite, PROMOTION_X + i * SQUARE_SIZE, PROMOTION_Y);
        }
    }

    fn to_delete(&self, shared: &Shareable) -> bool {
        self.to_delete || !shared.in_promotion
    }
}
