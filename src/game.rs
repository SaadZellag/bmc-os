use alloc::{
    boxed::Box,
    vec::{self, Vec},
};
use arrayvec::ArrayVec;
use cozy_chess::{Board, Move, Square};
use engine::{
    engine::{Engine, EngineOptions, MAX_DEPTH},
    handler::SearchHandler,
    search::{tt::TranspositionTable, SearchSharedState},
    utils::tablesize::TableSize,
    Eval,
};
use pc_keyboard::DecodedKey;
use ps2_mouse::MouseState;
use x86_64::instructions::hlt;

use crate::{
    display::{
        color::Color256,
        graphics::{clear_buffer, draw_sprite, flush_buffer, Rectangle, HEIGHT, WIDTH},
        sprite::Sprite,
    },
    entities::{
        is_checkmate, Button, ChessBoard, ColorSelector, DifficultySelector, EngineEval,
        PromotionDisplayer, Text, BOARD_X, BOARD_Y, BORDER_SIZE, SQUARE_SIZE,
    },
    events::add_event,
    load_sprite, set_pixel,
};

const MOUSE_WIDTH: usize = 7;
const MOUSE_HEIGHT: usize = 10;

const MOUSE: Sprite = load_sprite!("../sprites/Mouse.data", MOUSE_WIDTH);

struct Handler {
    res: Option<engine::SearchResult>,
    current_depth: u8,
    max_depth: u8,
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
    StartEngineSearch(u8),                                    // depth
    SetPlayerColor(cozy_chess::Color),
    SetEngineDepth(u8),
}

pub struct Shareable {
    pub board: Board,
    pub mouse_x: i16,
    pub mouse_y: i16,
    pub state: State,
    pub in_promotion: bool,
    pub engine_eval: Eval,
    pub engine_thinking: bool,
    pub user_color: cozy_chess::Color,
    pub engine_depth: u8,
}

impl Shareable {
    pub fn should_flip(&self) -> bool {
        self.user_color == cozy_chess::Color::Black
    }
}

pub struct Game<'a> {
    shared: Shareable,
    engine: Engine<'a, Handler>,
    history: Vec<u64>,
    entities: Vec<Box<dyn Entity>>,
}

impl SearchHandler for Handler {
    fn new_result(&mut self, result: engine::SearchResult) {
        self.res = Some(result);
    }

    fn should_stop(&self) -> bool {
        self.res
            .map(|res| res.stats.depth >= self.current_depth)
            .unwrap_or(false)
    }
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let board = Board::default();
        let options = EngineOptions {
            tt_size: TableSize::from_kb(10),
            depth: 128,
        };
        let search_shared = SearchSharedState {
            handler: Handler {
                res: None,
                current_depth: 1,
                max_depth: 3,
            },
            history: ArrayVec::new(),
            tt: TranspositionTable::new(TableSize::from_kb(10)),
            killers: [[None; 2]; MAX_DEPTH as usize],
        };
        let shared = Shareable {
            board: board.clone(),
            mouse_x: 0,
            mouse_y: 0,
            state: State::Menu,
            in_promotion: false,
            engine_eval: Eval::NEUTRAL,
            engine_thinking: false,
            user_color: cozy_chess::Color::White,
            engine_depth: search_shared.handler.max_depth,
        };
        let engine = Engine::new(board, options, search_shared);
        Self {
            shared,
            history: Vec::new(),
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
            Event::MouseInput(state) => self.handle_mouse_input(state),
            Event::KeyboardInput(_) => {}
            Event::StartGame => self.start_game(),
            Event::EndGame => self.end_game(),
            Event::ReturnToMenu => self.return_to_menu(),
            Event::PlayMove(mv) => self.play_move(*mv),
            Event::DisplayPromotion(from, to) => self.display_promotion(*from, *to),
            Event::Exit => loop {
                hlt()
            },
            Event::StartEngineSearch(depth) => self.start_engine_search(*depth),
            Event::SetPlayerColor(color) => self.shared.user_color = *color,
            Event::SetEngineDepth(depth) => {
                self.shared.engine_depth = *depth;
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

    fn handle_mouse_input(&mut self, state: &MouseState) {
        self.shared.mouse_x += state.get_x();
        self.shared.mouse_y -= state.get_y();

        self.shared.mouse_x = self.shared.mouse_x.clamp(0, (WIDTH - MOUSE_WIDTH) as i16);
        self.shared.mouse_y = self.shared.mouse_y.clamp(0, (HEIGHT - MOUSE_HEIGHT) as i16);
    }

    fn play_move(&mut self, mv: Move) {
        self.shared.in_promotion = false;

        let board = &mut self.shared.board;
        self.history.push(board.hash());
        board.play(mv);

        if board.side_to_move() != self.shared.user_color && !is_checkmate(board) {
            self.engine.set_position(board.clone(), &self.history);
            self.engine.mut_handler().res = None;
            add_event(Event::StartEngineSearch(1))
        }
    }

    fn display_promotion(&mut self, from: Square, to: Square) {
        if !self.shared.in_promotion {
            self.shared.in_promotion = true;
            self.entities
                .push(Box::new(PromotionDisplayer::new(from, to)));
        }
    }

    fn start_engine_search(&mut self, depth: u8) {
        if depth == 1 {
            self.engine
                .set_position(self.shared.board.clone(), &self.history);
            self.engine.mut_handler().max_depth = self.shared.engine_depth;
        }
        self.shared.engine_thinking = true;

        self.engine.mut_handler().current_depth = depth + 1;
        self.engine.best_move_starting(depth);
        self.shared.engine_eval = self.engine.handler().res.unwrap().eval;

        if depth >= self.engine.handler().max_depth {
            let mv = self.engine.handler().res.unwrap().best_move;
            add_event(Event::PlayMove(mv));
            self.shared.engine_thinking = false;
            return;
        }

        add_event(Event::StartEngineSearch(depth + 1))
    }

    fn start_game(&mut self) {
        self.entities.clear();
        self.shared.board = Board::default();
        self.shared.state = State::InGame;
        self.shared.engine_eval = Eval::NEUTRAL;
        self.engine.mut_handler().res = None;
        self.history.clear();

        self.entities.push(Box::new(ChessBoard::new()));
        self.entities.push(Box::new(EngineEval::new()));

        if self.shared.should_flip() {
            add_event(Event::StartEngineSearch(1))
        }
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

        self.entities.push(Box::new(Button::with_text(
            GAME_OVER,
            "GAME OVER",
            Event::ReturnToMenu,
        )));

        let (text, color) = match self.shared.board.side_to_move() == self.shared.user_color {
            true => ("YOU LOSE", Color256::RED),
            false => ("YOU WIN", Color256::GREEN),
        };

        let mut text = Text::new(GAME_RESULT, text);
        text.set_color(color);

        self.entities.push(Box::new(text));

        self.shared.state = State::GameOver;
    }

    fn return_to_menu(&mut self) {
        self.entities.clear();

        const START: Rectangle = Rectangle {
            x: (WIDTH - 80) / 2,
            y: 128,
            width: 80,
            height: 32,
        };

        const EXIT: Rectangle = Rectangle {
            x: (WIDTH - 80) / 2,
            y: 192,
            width: 80,
            height: 32,
        };

        let mut start = Button::with_text(START, "Start", Event::StartGame);
        start.set_color(Color256::GREEN);

        let mut exit = Button::with_text(EXIT, "Exit", Event::Exit);
        exit.set_color(Color256::RED);

        self.entities.push(Box::new(DifficultySelector::new()));
        self.entities.push(Box::new(ColorSelector::new()));
        self.entities.push(Box::new(start));
        self.entities.push(Box::new(exit));

        self.shared.state = State::Menu;
    }
}
