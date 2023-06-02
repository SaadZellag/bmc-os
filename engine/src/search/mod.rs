use arrayvec::ArrayVec;
use cozy_chess::{Board, Move};

use crate::{
    engine::{EngineOptions, MAX_DEPTH},
    handler::SearchHandler,
    search::tt::TranspositionTable,
};

pub mod move_ordering;
mod position;
pub mod searcher;
mod see;
pub mod tt;

#[derive(Debug, Clone)]
pub struct Node {
    pub board: Board,
    // TODO: Add NNUE here
}

pub const NUM_KILLERS: usize = 2;

pub struct SearchSharedState<H: SearchHandler> {
    pub handler: H,
    pub history: ArrayVec<u64, 256>,
    pub tt: TranspositionTable,
    pub killers: [[Option<Move>; NUM_KILLERS]; MAX_DEPTH as usize],
    // TODO: Add (possible)search parameters
}

impl<H> Default for SearchSharedState<H>
where
    H: SearchHandler + Default,
{
    fn default() -> Self {
        Self {
            handler: H::default(),
            history: ArrayVec::new(),
            tt: TranspositionTable::new(EngineOptions::default().tt_size),
            killers: [[None; 2]; MAX_DEPTH as usize],
        }
    }
}
