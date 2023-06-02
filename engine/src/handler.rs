use crate::SearchResult;

pub trait SearchHandler {
    fn new_result(&mut self, result: SearchResult);

    fn should_stop(&self) -> bool;

    fn default_handler() -> DefaultHandler {
        DefaultHandler {}
    }
}

// Handler that just lets the search run until completion
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultHandler {}

impl SearchHandler for DefaultHandler {
    fn new_result(&mut self, _: SearchResult) {}

    fn should_stop(&self) -> bool {
        false
    }
}
