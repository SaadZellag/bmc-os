use core::mem;

use cozy_chess::{BitBoard, Board, Move, Piece};

use crate::{
    engine::MAX_DEPTH,
    evaluation::nnue::NNUE,
    handler::SearchHandler,
    search::{
        position::Position,
        tt::{EntryType, TTEntry},
        SearchSharedState,
    },
    Eval, SearchResult, SearchStats,
};

mod killer;
mod quiese;

pub struct Searcher<'a> {
    root: Position,
    nnue: &'a NNUE,
}

impl<'a> Searcher<'a> {
    pub fn new(root: Board, nnue: &'a NNUE) -> Self {
        Self {
            root: Position::new(root, nnue),
            nnue,
        }
    }

    pub fn set_board(&mut self, board: Board) {
        self.root = Position::new(board, self.nnue);
    }

    pub fn search<H: SearchHandler>(
        &mut self,
        depth: u8,
        shared: &mut SearchSharedState<H>,
        quiese: bool,
        best_mv_last_search: Option<Move>,
    ) -> Option<SearchResult> {
        let mut best_mv = best_mv_last_search;
        let mut stats = SearchStats::default();
        stats.depth = depth;
        stats.sel_depth = depth;

        let mut best_eval = if let Some(best_mv) = best_mv {
            let copy = self.root.make_move(best_mv, self.nnue);
            -self.search_node(
                &copy,
                depth - 1,
                &mut stats,
                shared,
                Eval::WORST_EVAL,
                Eval::BEST_EVAL,
                quiese,
            )?
        } else {
            Eval::MIN
        };

        for mv in self.root.possible_moves(shared) {
            // If the current move explored was the last best move, skip it since it was already searched
            match best_mv {
                Some(best_mv) if best_mv == mv => continue,
                _ => {}
            }

            let copy = self.root.make_move(mv, self.nnue);
            let eval = -self.search_node(
                &copy,
                depth - 1,
                &mut stats,
                shared,
                Eval::WORST_EVAL,
                -best_eval,
                quiese,
            )?;

            // println!("{} {:?} {:?}", mv, eval, best_eval);

            if eval > best_eval {
                best_eval = eval;
                best_mv = Some(mv);
            }
        }

        let mut moves = unsafe { [mem::zeroed(); MAX_DEPTH as usize] };
        let mut current_mv = best_mv.expect("No moves found");
        let mut current_board = self.root.clone();

        for i in 0..stats.depth {
            moves[i as usize] = current_mv;
            current_board = current_board.make_move(current_mv, self.nnue);
            if let Some(ttentry) = shared.tt.get(&current_board) {
                // assert_eq!(ttentry.flag, EntryType::Exact);
                current_mv = ttentry.mv;
                if !current_board.board().is_legal(current_mv) {
                    panic!(
                        "{} received move {} from tt entry {:?} | Board hash {}",
                        current_board.board(),
                        current_mv,
                        ttentry,
                        current_board.board().hash()
                    );
                }

                if ttentry.flag != EntryType::Exact {
                    break;
                }
            } else {
                break;
            }
        }

        best_mv.map(|mv| SearchResult {
            best_move: mv,
            eval: best_eval,
            stats,
            hashfull: 0,
            pv: moves,
        })
    }

    // https://www.chessprogramming.org/Negamax
    // https://www.chessprogramming.org/Alpha-Beta
    // https://en.wikipedia.org/wiki/Negamax
    // https://www.chessprogramming.org/Principal_Variation
    fn search_node<H: SearchHandler>(
        &mut self,
        pos: &Position,
        mut depth: u8,
        stats: &mut SearchStats,
        shared: &mut SearchSharedState<H>,
        mut alpha: Eval,
        mut beta: Eval,
        quiese: bool,
    ) -> Option<Eval> {
        if shared.handler.should_stop() {
            return None;
        }

        let orig_alpha = alpha;
        let board = pos.board();

        // Checking for transposition
        let ttentry = shared.tt.get(pos);
        if let Some(ttentry) = ttentry {
            if ttentry.depth >= depth {
                stats.tbl_hits += 1;
                match ttentry.flag {
                    EntryType::Exact => return Some(ttentry.eval),
                    EntryType::LowerBound => alpha = alpha.max(ttentry.eval),
                    EntryType::UpperBound => beta = beta.min(ttentry.eval),
                    EntryType::Invalid => {
                        panic!("Invalid entry") // Just in case
                    }
                }
                if alpha >= beta {
                    return Some(ttentry.eval);
                }
            }
        }

        let hash = board.hash();
        shared.history.push(hash);

        // Check extension
        // https://www.chessprogramming.org/Check_Extensions
        let in_check = board.checkers().len() != 0;
        if in_check {
            depth += 1;
        }

        macro_rules! _return {
            ($eval:expr) => {
                shared.history.pop();
                return $eval;
            };
        }

        // Check for repetition
        if self.repetitions(shared, hash) > 0 {
            _return!(Some(Eval::NEUTRAL));
        }

        stats.nodes_visited += 1;

        if depth == 0 {
            _return!(if quiese {
                self.quiese(pos, 1, stats, shared, alpha, beta)
            } else {
                Some(pos.eval(&self.nnue))
            });
        }

        let mut itt = pos.possible_moves(shared);
        if itt.len() == 0 {
            _return!(if quiese {
                self.quiese(pos, 1, stats, shared, alpha, beta)
            } else {
                match board.checkers().len() {
                    0 => Some(Eval::NEUTRAL),
                    _ => Some(Eval::MatedIn(pos.ply())),
                }
            });
        }

        // Null move pruning
        const R: u8 = 2;

        // Checking whether NMP is applicable
        let our_pieces = board.colors(board.side_to_move());
        let pawns = board.pieces(Piece::Pawn);
        let our_pawns = our_pieces & pawns;
        let only_pawns = our_pawns.len() == our_pieces.len() - 1; // the king is always there

        let do_nmp = our_pieces.len() >= 8 && !only_pawns; // The 8 is just me who picked it

        if do_nmp {
            if let Some(new_board) = pos.null_move() {
                let score = -self.search_node(
                    &new_board,
                    depth.saturating_sub(R + 1),
                    stats,
                    shared,
                    -beta,
                    -beta + Eval::UNIT,
                    quiese,
                )?;

                if score >= beta {
                    _return!(Some(score));
                }
            }
        }

        let mut b_search_pv = true;

        let mut score = Eval::MIN;
        let mut best_mv = unsafe { mem::zeroed() };

        if let Some(ttentry) = ttentry {
            let mv = ttentry.mv;
            assert!(
                board.is_legal(mv),
                "Got an invalid move {} for position {} from the TT",
                mv,
                board
            );

            itt.remove_move(mv);

            best_mv = mv;
            let copy = pos.make_move(mv, self.nnue);
            score = -self.search_node(&copy, depth - 1, stats, shared, -beta, -alpha, quiese)?;

            if score > alpha {
                alpha = score;
                b_search_pv = false;
            }

            if alpha >= beta {
                itt.allow_only(BitBoard::EMPTY);
            }
        }

        for mv in itt {
            let copy = pos.make_move(mv, self.nnue);

            let mut current_score;

            if b_search_pv {
                current_score =
                    -self.search_node(&copy, depth - 1, stats, shared, -beta, -alpha, quiese)?;
            } else {
                current_score = -self.search_node(
                    &copy,
                    depth - 1,
                    stats,
                    shared,
                    -alpha - Eval::UNIT,
                    -alpha,
                    quiese,
                )?;
                if current_score > alpha && current_score < beta {
                    current_score = -self.search_node(
                        &copy,
                        depth - 1,
                        stats,
                        shared,
                        -beta,
                        -alpha,
                        quiese,
                    )?;
                }
            };

            if current_score > score {
                score = current_score;
                best_mv = mv;
            }

            if score > alpha {
                alpha = score;
                b_search_pv = false;
            }

            if alpha >= beta {
                // Only add killer if quiet move
                if board.piece_on(mv.to).is_none() {
                    self.insert_killer(pos.ply() as usize, mv, shared);
                }
                break;
            }
        }

        // Storing in tt table
        let flag = if score <= orig_alpha {
            EntryType::UpperBound
        } else if score >= beta {
            EntryType::LowerBound
        } else {
            EntryType::Exact
        };

        let ttentry = TTEntry {
            hash: board.hash(),
            flag,
            depth: depth,
            eval: score,
            mv: best_mv,
        };

        shared.tt.set(pos, ttentry);

        _return!(Some(score));
    }

    fn repetitions<H: SearchHandler>(&self, shared: &SearchSharedState<H>, hash: u64) -> usize {
        shared
            .history
            .iter()
            .rev()
            .step_by(2)
            .skip(1)
            .filter(|&&h| h == hash)
            .count()
    }
}
