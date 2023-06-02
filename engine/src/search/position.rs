use cozy_chess::{Board, GameStatus, Move};

use crate::{
    evaluation::{
        evaluator::Evaluator,
        nnue::{NNUEAccumulator, NNUE},
    },
    handler::SearchHandler,
    search::{move_ordering::MoveList, SearchSharedState},
    Eval,
};

const USE_NNUE: bool = true;

#[derive(Debug, Clone)]
pub struct Position {
    board: Board,
    acc: NNUEAccumulator,
    ply: u8,
}

impl Position {
    pub fn new(board: Board, nnue: &NNUE) -> Self {
        let acc = NNUEAccumulator::new(&board, nnue);
        Self { board, acc, ply: 0 }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn acc(&self) -> &NNUEAccumulator {
        &self.acc
    }

    pub fn ply(&self) -> u8 {
        self.ply
    }

    pub fn possible_moves<H: SearchHandler>(&self, shared: &mut SearchSharedState<H>) -> MoveList {
        let mut moves = MoveList::new(&self.board);
        moves.order_moves(&self, shared);
        moves
    }

    pub fn possible_captures<H: SearchHandler>(
        &self,
        shared: &mut SearchSharedState<H>,
    ) -> MoveList {
        let mut moves = MoveList::with_mask(&self.board, self.board.occupied());
        moves.order_moves(&self, shared);
        moves
    }

    pub fn make_move(&self, mv: Move, nnue: &NNUE) -> Self {
        let mut new_board = self.board.clone();
        new_board.play_unchecked(mv);

        let acc = if USE_NNUE {
            self.acc.update(nnue, &self.board, &new_board, mv)
        } else {
            self.acc.clone()
        };

        Self {
            board: new_board,
            acc,
            ply: self.ply + 1,
        }
    }

    pub fn null_move(&self) -> Option<Self> {
        Some(Self {
            board: self.board.null_move()?,
            acc: self.acc.clone(),
            ply: self.ply + 1,
        })
    }

    pub fn eval(&self, nnue: &NNUE) -> Eval {
        if !USE_NNUE {
            let mut eval = Evaluator {}.evaluate(&self.board);
            if let Eval::MatedIn(_) = eval {
                eval = Eval::MatedIn(self.ply);
            }
            return eval;
        }
        match self.board.status() {
            GameStatus::Ongoing => {}
            GameStatus::Drawn => return Eval::NEUTRAL,
            GameStatus::Won => return Eval::MatedIn(self.ply),
        };

        nnue.eval(&self.acc, self.board.side_to_move())
    }
}
