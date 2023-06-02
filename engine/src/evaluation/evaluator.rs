use cozy_chess::{Board, GameStatus};

use crate::Eval;

#[derive(Debug, Clone)]
pub struct Evaluator {
    // Empty for now
}

impl Evaluator {
    pub fn evaluate(&self, board: &Board) -> Eval {
        match board.status() {
            GameStatus::Ongoing => {}
            GameStatus::Drawn => return Eval::NEUTRAL,
            GameStatus::Won => return Eval::WORST_EVAL,
        };

        let mut total = 0;
        macro_rules! add_eval {
            ($($name:ident),*) => {
                $(total += $crate::evaluation::$name::eval_board(board);)*
            };
        }

        add_eval!(piece_eval, map_eval);

        Eval::CentiPawn(total)
        // let acc = NNUEAccumulator::new(board, &NNUE);
        // NNUE.eval(&acc, board.side_to_move())
    }
}

#[test]
fn test_positions() {
    let boards = PositionGenerator::new().take(10000);

    for board in boards {
        if board.status() == BoardStatus::Stalemate {
            continue;
        }
        let eval = Evaluator {}.evaluate(&board);
        if let Some(new_board) = board.null_move() {
            assert_eq!(
                -eval,
                Evaluator {}.evaluate(&new_board),
                "Positon {} has different evals for both sides",
                board,
            );
        }
    }
}
