use arrayvec::ArrayVec;
use cozy_chess::{BitBoard, Board, Move, Piece};

use crate::{
    engine::MAX_DEPTH,
    handler::SearchHandler,
    search::{position::Position, see::static_exchange, SearchSharedState},
    EvalType,
};

// victim attacker
// King should NEVER be a victim
const MVV_LVA: [[i8; 6]; 5] = [
    [9, 8, 7, 6, 5, 4],       // Pawn
    [19, 18, 17, 16, 15, 14], // Knight
    [29, 28, 27, 26, 25, 24], // Bishop
    [39, 38, 37, 36, 35, 34], // Rook
    [49, 48, 47, 46, 45, 44], // Queen
];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum MoveScore {
    None,
    Killer,
    // MVVLVA(i8),
    SEE(EvalType),
}

fn score_move<H: SearchHandler>(
    pos: &Position,
    mv: Move,
    shared: &SearchSharedState<H>,
) -> MoveScore {
    let board = pos.board();
    let attacker = board.piece_on(mv.from).expect("Invalid board");
    let victim = board.piece_on(mv.to);

    for killer in shared.killers[pos.ply() as usize] {
        if Some(mv) == killer {
            return MoveScore::Killer;
        }
    }
    if let Some(_) = victim {
        // MoveScore::MVVLVA(MVV_LVA[victim as usize][attacker as usize])
        MoveScore::SEE(static_exchange(board, mv))
    } else {
        MoveScore::None
    }
}

pub struct MoveList {
    moves: ArrayVec<Move, 218>, // Max number of legal moves
}

impl MoveList {
    pub fn new(board: &Board) -> Self {
        Self::with_mask(board, !BitBoard::EMPTY)
    }

    pub fn with_mask(board: &Board, mask: BitBoard) -> Self {
        let mut moves = ArrayVec::new();

        board.generate_moves(|mvs| {
            for mv in mvs {
                if mask.has(mv.to) {
                    moves.push(mv);
                }
            }
            false
        });

        moves.reverse();

        Self { moves }
    }

    pub fn order_moves<H>(&mut self, pos: &Position, shared: &mut SearchSharedState<H>)
    where
        H: SearchHandler,
    {
        // Best moves have higher value and we want them last
        // Using pop() to get the moves
        introsort::sort_by(&mut self.moves, &|a, b| {
            score_move(pos, *a, shared).cmp(&score_move(pos, *b, shared))
        });
        // self.moves
        //     .sort_by_cached_key(|mv| score_move(pos, *mv, shared));
    }

    pub fn remove_move(&mut self, mv: Move) {
        self.moves.retain(|m| m != &mv);
    }

    pub fn allow_only(&mut self, mask: BitBoard) {
        self.moves
            .retain(|mv| mv.to.bitboard() & mask != BitBoard::EMPTY)
    }
}

impl Iterator for MoveList {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.moves.pop()
    }
}

impl ExactSizeIterator for MoveList {
    fn len(&self) -> usize {
        self.moves.len()
    }
}

#[test]
fn test_positions() {
    // To make sure that the ordered movegen generates the same amount of moves as the normal one
    use crate::utils::positiongen::PositionGenerator;

    for board in PositionGenerator::new().take(100) {
        let movegen_moves = MoveGen::new_legal(&board);
        let orderedmovegen_moves = MoveList::new(&board);

        assert_eq!(movegen_moves.len(), orderedmovegen_moves.len());
        assert_eq!(
            movegen_moves.into_iter().collect::<Vec<_>>().len(),
            orderedmovegen_moves.into_iter().collect::<Vec<_>>().len()
        );
    }
}
