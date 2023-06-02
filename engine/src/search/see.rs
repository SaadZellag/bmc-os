use cozy_chess::{
    get_bishop_moves, get_king_moves, get_knight_moves, get_pawn_attacks, get_rook_moves, BitBoard,
    Board, Color, Move, Piece, Square,
};

use crate::EvalType;

const VALUES: [EvalType; 6] = [100, 325, 350, 500, 900, 100000];

pub fn static_exchange(board: &Board, mv: Move) -> EvalType {
    let mut scores = [0; 32];
    let mut score_index = 0;
    let mut stm = board.side_to_move();

    let src = mv.from;
    let dest = mv.to;
    let initial_capture = board.piece_on(dest).unwrap();

    // Step 1: Finding the obvious attackers
    let mut blockers = board.occupied() & !src.bitboard();
    let mut attackers = BitBoard::EMPTY;

    attackers |= get_pawn_attacks(dest, Color::White)
        & blockers
        & board.colors(Color::Black)
        & board.pieces(Piece::Pawn);

    attackers |= get_pawn_attacks(dest, Color::Black)
        & blockers
        & board.colors(Color::White)
        & board.pieces(Piece::Pawn);

    attackers |= get_knight_moves(dest) & blockers & board.pieces(Piece::Knight);

    attackers |= get_bishop_moves(dest, blockers)
        & blockers
        & (board.pieces(Piece::Bishop) | board.pieces(Piece::Queen));

    attackers |= get_rook_moves(dest, blockers)
        & blockers
        & (board.pieces(Piece::Rook) | board.pieces(Piece::Queen));

    attackers |= get_king_moves(dest) & blockers & board.pieces(Piece::King);

    // Step 2: Simulate the initial capture
    scores[score_index] = VALUES[initial_capture as usize];
    score_index += 1;
    stm = !stm;
    let mut attacked_piece = board.piece_on(src).unwrap();

    'outer: while score_index < 32 {
        for piece in Piece::ALL {
            let our_attackers = attackers & board.colors(stm) & board.pieces(piece);
            let mut our_attackers = our_attackers.into_iter();

            if let Some(square) = our_attackers.next() {
                scores[score_index] = VALUES[attacked_piece as usize] - scores[score_index - 1];
                stm = !stm;
                score_index += 1;

                if attacked_piece == Piece::King {
                    break 'outer;
                }

                attackers ^= square.bitboard();
                blockers ^= square.bitboard();

                attacked_piece = piece;

                // Step 3: Adding hidden pieces
                if piece == Piece::Rook || piece == Piece::Queen {
                    attackers |= get_rook_moves(square, blockers)
                        & blockers
                        & (board.pieces(Piece::Rook) | board.pieces(Piece::Queen));
                }

                if piece == Piece::Pawn || piece == Piece::Bishop || piece == Piece::Queen {
                    attackers |= get_bishop_moves(square, blockers)
                        & blockers
                        & (board.pieces(Piece::Bishop) | board.pieces(Piece::Queen));
                }

                // if piece == Piece::Pawn {
                //     attackers |= get_pawn_attacks(square, Color::White, blockers)
                //         & board.color_combined(Color::Black);

                //     attackers |= get_pawn_attacks(square, Color::Black, blockers)
                //         & board.color_combined(Color::White);

                //     println!("Attackers after pawn");
                //     println!("{}", attackers);

                //     println!("Blockers after pawn")
                // }
                continue 'outer;
            }
        }
        break;
    }

    while score_index > 1 {
        score_index -= 1;
        if scores[score_index - 1] > -scores[score_index] {
            scores[score_index - 1] = -scores[score_index];
        }
    }

    scores[0]
}
