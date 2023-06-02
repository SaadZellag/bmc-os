use cozy_chess::Move;

use crate::{
    handler::SearchHandler,
    search::{searcher::Searcher, SearchSharedState, NUM_KILLERS},
};

impl<'a> Searcher<'a> {
    pub(crate) fn insert_killer<H: SearchHandler>(
        &mut self,
        ply: usize,
        mv: Move,
        shared: &mut SearchSharedState<H>,
    ) {
        let entries = &mut shared.killers[ply];
        if entries[0] == Some(mv) {
            return; // Don't add the same one
        }

        if entries[0] == entries[1] && entries[0] != None {
            panic!("The killers have the same value: {:?}", entries[0]);
        }

        // Pushing it
        for i in (1..NUM_KILLERS).rev() {
            entries[i] = entries[i - 1];
        }
        entries[0] = Some(mv);
    }
}
