use crate::{
    handler::SearchHandler,
    search::{position::Position, searcher::Searcher, see::static_exchange, SearchSharedState},
    Eval, SearchStats,
};

impl<'a> Searcher<'a> {
    // https://www.chessprogramming.org/Quiescence_Search
    pub(crate) fn quiese<H: SearchHandler>(
        &mut self,
        pos: &Position,
        current_depth: u8,
        stats: &mut SearchStats,
        shared: &mut SearchSharedState<H>,
        mut alpha: Eval,
        beta: Eval,
    ) -> Option<Eval> {
        if shared.handler.should_stop() {
            return None;
        }

        let itt = pos.possible_captures(shared);

        let current_score = pos.eval(self.nnue);
        stats.nodes_visited += 1;

        if itt.len() == 0 {
            return Some(current_score);
        }

        stats.sel_depth = stats.sel_depth.max(current_depth + stats.depth);

        if current_score >= beta {
            return Some(beta);
        }
        if current_score > alpha {
            alpha = current_score;
        }

        for mv in itt {
            if static_exchange(pos.board(), mv) < 0 {
                continue;
            }

            let copy = pos.make_move(mv, self.nnue);
            let eval = -self.quiese(&copy, current_depth + 1, stats, shared, -beta, -alpha)?;

            if eval >= beta {
                return Some(beta);
            }

            if eval > alpha {
                alpha = eval;
            }
        }

        Some(alpha)
    }
}
