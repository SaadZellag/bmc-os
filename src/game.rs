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
use x86_64::instructions::hlt;

use crate::{
    display::{
        color::Color256,
        graphics::{clear_buffer, draw_sprite, flush_buffer, Rectangle},
        sprite::Sprite,
    },
    entities::{
        Button, ChessBoard, PromotionDisplayer, Text, BOARD_X, BOARD_Y, BORDER_SIZE, SQUARE_SIZE,
    },
    load_sprite,
};

const MOUSE: Sprite = load_sprite!("../sprites/Mouse.data", 7);

struct Handler {
    res: Option<engine::SearchResult>,
}

#[derive(PartialEq, Eq)]
pub enum State {
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
    Exit,
    PlayMove(cozy_chess::Move),
    DisplayPromotion(cozy_chess::Square, cozy_chess::Square), // From to dest Square for the struct
}

pub struct Shareable {
    pub board: Board,
    pub mouse_x: i16,
    pub mouse_y: i16,
    pub state: State,
    pub in_promotion: bool,
}

pub struct Game<'a> {
    shared: Shareable,
    engine: Engine<'a, Handler>,
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
                state: State::Menu,
                in_promotion: false,
            },
            engine,
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
                self.shared.in_promotion = false;
                self.shared.board.play(*mv);
            }
            Event::DisplayPromotion(from, to) => {
                if !self.shared.in_promotion {
                    self.shared.in_promotion = true;
                    self.entities
                        .push(Box::new(PromotionDisplayer::new(*from, *to)));
                }
            }
            Event::Exit => loop {
                hlt()
            },
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
        self.shared.board = Board::default();
        self.shared.state = State::InGame;

        self.entities.push(Box::new(ChessBoard::new()));
    }

    fn end_game(&mut self) {
        const GAME_OVER: Rectangle = Rectangle {
            x: BOARD_X - BORDER_SIZE,
            y: BOARD_Y - 32,
            width: 80,
            height: 16,
        };

        const GAME_RESULT: Rectangle = Rectangle {
            x: BOARD_X + 80 + BORDER_SIZE - 1,
            y: BOARD_Y - 32,
            width: 80,
            height: 16,
        };

        self.entities.push(Box::new(Button::new(
            GAME_OVER,
            "GAME OVER",
            Event::ReturnToMenu,
        )));

        let (text, color) = match self.shared.board.side_to_move() {
            cozy_chess::Color::White => ("YOU LOSE", Color256::RED),
            cozy_chess::Color::Black => ("YOU WIN", Color256::GREEN),
        };

        let mut text = Text::new(GAME_RESULT, text);
        text.set_color(color);

        self.entities.push(Box::new(text));

        self.shared.state = State::GameOver;
    }

    fn return_to_menu(&mut self) {
        self.entities.clear();

        const START: Rectangle = Rectangle {
            x: 128,
            y: 64,
            width: 64,
            height: 32,
        };

        const EXIT: Rectangle = Rectangle {
            x: 128,
            y: 128,
            width: 64,
            height: 32,
        };

        let mut start = Button::new(START, "Start", Event::StartGame);
        start.set_color(Color256::GREEN);
        self.entities.push(Box::new(start));

        let mut exit = Button::new(EXIT, "Exit", Event::Exit);
        exit.set_color(Color256::RED);
        self.entities.push(Box::new(exit));

        self.shared.state = State::Menu;
    }
}
