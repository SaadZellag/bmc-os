use cozy_chess::Board;

use crate::{
    evaluation::nnue::NNUE,
    handler::SearchHandler,
    search::{searcher::Searcher, SearchSharedState},
    utils::tablesize::TableSize,
    Eval, SearchResult,
};

pub const MAX_DEPTH: u8 = 128;

pub struct Engine<'a, H: SearchHandler> {
    shared: SearchSharedState<H>,
    searcher: Searcher<'a>,
    options: EngineOptions,
}

#[derive(Debug, Clone, Copy)]
pub struct EngineOptions {
    pub tt_size: TableSize, // TODO: Change this to struct to remove conversion error
    pub depth: u8,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            tt_size: TableSize::default(),
            depth: MAX_DEPTH,
        }
    }
}

pub const EVALUATOR: NNUE = NNUE::new();

impl<'a, H: SearchHandler> Engine<'a, H> {
    pub fn new(position: Board, options: EngineOptions, shared: SearchSharedState<H>) -> Self {
        Self {
            shared,
            searcher: Searcher::new(position, &EVALUATOR),
            options,
        }
    }

    pub fn with_nnue(
        position: Board,
        options: EngineOptions,
        shared: SearchSharedState<H>,
        nnue: &'a NNUE,
    ) -> Self {
        Self {
            shared,
            searcher: Searcher::new(position, &nnue),
            options,
        }
    }

    pub fn best_move(&mut self) -> Option<SearchResult> {
        self.best_move_starting(0)
    }

    pub fn best_move_starting(&mut self, start: u8) -> Option<SearchResult> {
        // TODO: Checking if only 1 move possible, then playing that

        // Running depth 1 without quiese at least have a move
        // TODO: Remove this when search explosion problem is fixed
        let mut quiese = true;

        let mut res: Option<SearchResult> = None;

        for depth in start..=self.options.depth {
            if let Some(most_recent) =
                self.searcher
                    .search(depth, &mut self.shared, quiese, res.map(|r| r.best_move))
            {
                quiese = true;
                self.shared.handler.new_result(most_recent);
                res = Some(most_recent);

                // Preventing from looking further if mate is forced
                match most_recent.eval {
                    Eval::MateIn(_) | Eval::MatedIn(_) => break,
                    _ => {}
                }
            } else {
                break;
            }
        }

        res
    }

    pub fn handler(&self) -> &H {
        &self.shared.handler
    }

    pub fn mut_handler(&mut self) -> &mut H {
        &mut self.shared.handler
    }

    pub fn set_position(&mut self, board: Board, history: &[u64]) {
        self.searcher.set_board(board);
        self.shared.history = arrayvec::ArrayVec::from_iter(history.iter().cloned())
    }

    pub fn set_handler(&mut self, handler: H) {
        self.shared.handler = handler;
    }
}
