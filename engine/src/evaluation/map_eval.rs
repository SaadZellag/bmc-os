// yoinked from https://github.com/mvanthoor/rustic/blob/master/src/evaluation/psqt.rs

use cozy_chess::{Board, Color, Piece, Square};

use crate::{utils::chessutils::SquareUtils, EvalType};

type Psqt = [EvalType; 64];
#[rustfmt::skip]
const KING_MG: Psqt = [
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,   -10,  -10,    0,    0,    0,
    0,    0,    20,   -10,  -10,    0,   20,    0,
];

#[rustfmt::skip]
const QUEEN_MG: Psqt = [
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -10,   -5,   10,   10,   10,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30 
];

#[rustfmt::skip]
const ROOK_MG: Psqt = [
    0,   0,   0,   0,   0,   0,   0,   0,
   15,  15,  15,  20,  20,  15,  15,  15,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,  10,  10,  10,   0,   0
];

#[rustfmt::skip]
const BISHOP_MG: Psqt = [
    -20,    0,    0,    0,    0,    0,    0,  -20,
    -15,    0,    0,    0,    0,    0,    0,  -15,
    -10,    0,    0,    5,    5,    0,    0,  -10,
    -10,   10,   10,   30,   30,   10,   10,  -10,
      5,    5,   10,   25,   25,   10,    5,    5,
      5,    5,    5,   10,   10,    5,    5,    5,
    -10,    5,    5,   10,   10,    5,    5,  -10,
    -20,  -10,  -10,  -10,  -10,  -10,  -10,  -20
];

#[rustfmt::skip]
const KNIGHT_MG: Psqt = [
    -20, -10,  -10,  -10,  -10,  -10,  -10,  -20,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   10,   15,   15,   15,   -5,  -10,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,   0,  -10,  -10,  -10,  -10,    0,  -20
];

#[rustfmt::skip]
const PAWN_MG: Psqt = [
     0,   0,   0,   0,   0,   0,   0,   0,
    60,  60,  60,  60,  70,  60,  60,  60,
    40,  40,  40,  50,  60,  40,  40,  40,
    20,  20,  20,  40,  50,  20,  20,  20,
     5,   5,  15,  30,  40,  10,   5,   5,
     5,   5,  10,  20,  30,   5,   5,   5,
     5,   5,   5, -30, -30,   5,   5,   5,
     0,   0,   0,   0,   0,   0,   0,   0
];

fn map_white(sq: Square) -> usize {
    sq.flip() as usize
}

fn map_black(sq: Square) -> usize {
    sq as usize
}

pub fn eval_board(board: &Board) -> EvalType {
    let mut total = 0;
    let white_pieces = board.colors(Color::White);
    let black_pieces = board.colors(Color::Black);

    macro_rules! opp_terms {
            ($map:ident $mapper:ident $op:tt $bb:expr) => {
                for sq in $bb.into_iter() {
                    total $op $map[$mapper(sq)];
                }
            };
        }

    macro_rules! apply_terms {
        ($($map:ident $piece:ident),*) => {
                $({
                    let pieces = board.pieces(Piece::$piece);
                    opp_terms!($map map_white += (pieces & white_pieces));
                    opp_terms!($map map_black -= (pieces & black_pieces));
            })*
            };
    }

    apply_terms!(
        PAWN_MG Pawn,
        BISHOP_MG Bishop,
        KNIGHT_MG Knight,
        ROOK_MG Rook,
        QUEEN_MG Queen,
        KING_MG King
    );

    total
        * if board.side_to_move() == Color::White {
            1
        } else {
            -1
        }
}
