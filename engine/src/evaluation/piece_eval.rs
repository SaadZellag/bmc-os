use cozy_chess::{Board, Color, Piece};

use crate::EvalType;

const QUEEN_VALUE: EvalType = 900;
const ROOK_VALUE: EvalType = 500;
const BISHOP_VALUE: EvalType = 300;
const KNIGHT_VALUE: EvalType = 300;
const PAWN_VALUE: EvalType = 100;

pub fn eval_board(board: &Board) -> EvalType {
    let mut total = 0;
    let white_pieces = board.colors(Color::White);
    let black_pieces = board.colors(Color::Black);

    macro_rules! add_terms {
            ($($eval:ident $piece:ident),*) => {
                $({
                    let pieces = board.pieces(Piece::$piece);
                    total += $eval * (pieces & white_pieces).len() as EvalType;
                    total -= $eval * (pieces & black_pieces).len() as EvalType;
            })*
            }
        }

    add_terms!(
        QUEEN_VALUE Queen,
        ROOK_VALUE Rook,
        BISHOP_VALUE Bishop,
        KNIGHT_VALUE Knight,
        PAWN_VALUE Pawn
    );

    total
        * if board.side_to_move() == Color::White {
            1
        } else {
            -1
        }
}
