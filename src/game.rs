use alloc::{boxed::Box, vec::Vec};
use arrayvec::ArrayVec;
use cozy_chess::{Board, Move, Square};
use engine::{
    engine::{Engine, EngineOptions, MAX_DEPTH},
    handler::SearchHandler,
    search::{tt::TranspositionTable, SearchSharedState},
    utils::tablesize::TableSize,
};
use pc_keyboard::DecodedKey;
use ps2_mouse::MouseState;

use crate::{
    display::{
        graphics::{clear_buffer, draw_sprite, flush_buffer},
        sprite::Sprite,
    },
    entities::ChessBoard,
    load_sprite,
};

const MOUSE: Sprite = load_sprite!("../sprites/Mouse.data", 7);
const chessboard: Sprite = load_sprite!("../sprites/chessboard.data", 160);

struct Handler {
    res: Option<engine::SearchResult>,
}

enum State {
    Menu,
    InGame,
    GameOver,
}

pub trait Entity {
    fn handle_event(&mut self, event: &Event, shared: &Shareable);

    fn draw(&self, shared: &Shareable);

    fn to_delete(&self, shared: &Shareable) -> bool;
}

#[derive(Debug, Clone)]
pub enum Event {
    MouseInput(MouseState),
    KeyboardInput(DecodedKey),
    StartGame,
    EndGame,
    ReturnToMenu,
    PlayMove(Move), // From, To
}

pub struct Shareable {
    pub board: Board,
    pub mouse_x: i16,
    pub mouse_y: i16,
}

pub struct Game<'a> {
    shared: Shareable,
    engine: Engine<'a, Handler>,
    state: State,
    entities: Vec<Box<dyn Entity>>,
}

impl SearchHandler for Handler {
    fn new_result(&mut self, result: engine::SearchResult) {
        self.res = Some(result);
    }

    fn should_stop(&self) -> bool {
        self.res.map(|r| r.stats.depth >= 7).unwrap_or(false)
    }
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let board = Board::default();
        let options = EngineOptions {
            tt_size: TableSize::from_kb(10),
            depth: 128,
        };
        let shared = SearchSharedState {
            handler: Handler { res: None },
            history: ArrayVec::new(),
            tt: TranspositionTable::new(TableSize::from_kb(10)),
            killers: [[None; 2]; MAX_DEPTH as usize],
        };
        let engine = Engine::new(board.clone(), options, shared);
        Self {
            shared: Shareable {
                board,
                mouse_x: 0,
                mouse_y: 0,
            },
            engine,
            state: State::Menu,
            entities: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        let mut indexes = Vec::new();
        for (i, entity) in self.entities.iter_mut().enumerate() {
            entity.handle_event(event, &self.shared);

            if entity.to_delete(&self.shared) {
                indexes.push(i);
            }
        }

        for i in indexes.into_iter().rev() {
            self.entities.remove(i);
        }

        match event {
            Event::MouseInput(state) => {
                self.shared.mouse_x += state.get_x();
                self.shared.mouse_y -= state.get_y();

                self.shared.mouse_x = self.shared.mouse_x.clamp(0, 313);
                self.shared.mouse_y = self.shared.mouse_y.clamp(0, 230);
            }
            Event::KeyboardInput(_) => {}
            Event::StartGame => self.start_game(),
            Event::EndGame => self.end_game(),
            Event::ReturnToMenu => self.return_to_menu(),
            Event::PlayMove(mv) => {
                // TODO: Handle promotion
                self.shared.board.play(*mv);
            }
        }
    }

    pub fn draw(&self) {
        clear_buffer();
        for entity in self.entities.iter() {
            entity.draw(&self.shared);
        }
        draw_sprite(
            &MOUSE,
            self.shared.mouse_x as usize,
            self.shared.mouse_y as usize,
        );
        flush_buffer();
    }

    fn start_game(&mut self) {
        self.entities.clear();

        self.entities.push(Box::new(ChessBoard::new()));
    }

    fn end_game(&mut self) {}

    fn return_to_menu(&mut self) {}
}
