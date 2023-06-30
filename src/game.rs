use alloc::{boxed::Box, vec::Vec};
use arrayvec::ArrayVec;
use cozy_chess::Board;
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

trait Entity {
    fn handle_event(&mut self, event: &Event);

    fn draw(&self);

    fn to_delete(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Event {
    MouseInput(MouseState),
    KeyboardInput(DecodedKey),
    StartGame,
    EndGame,
    ReturnToMenu,
}

pub struct Game<'a> {
    board: Board,
    engine: Engine<'a, Handler>,
    pub mouse_pos: (i16, i16),
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
            board,
            engine,
            mouse_pos: (0, 0),
            state: State::Menu,
            entities: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        for entity in self.entities.iter_mut() {
            entity.handle_event(event);
        }

        match event {
            Event::MouseInput(state) => {
                self.mouse_pos.0 += state.get_x();
                self.mouse_pos.1 -= state.get_y();

                self.mouse_pos.0 = self.mouse_pos.0.clamp(0, 313);
                self.mouse_pos.1 = self.mouse_pos.1.clamp(0, 230);
            }
            Event::KeyboardInput(_) => {}
            Event::StartGame => self.start_game(),
            Event::EndGame => self.end_game(),
            Event::ReturnToMenu => self.return_to_menu(),
        }
    }

    pub fn draw(&self) {
        clear_buffer();
        for entity in self.entities.iter() {
            entity.draw();
        }
        draw_sprite(&MOUSE, self.mouse_pos.0 as usize, self.mouse_pos.1 as usize);
        flush_buffer();
    }

    fn start_game(&mut self) {}

    fn end_game(&mut self) {}

    fn return_to_menu(&mut self) {}
}
