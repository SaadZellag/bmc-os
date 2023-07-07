use core::{array, f64::consts};

use crate::{
    display::{
        color::Color256,
        graphics::{
            contains_point, draw_shape, draw_sprite, draw_text, Rectangle, CHAR_HEIGHT, CHAR_WIDTH,
            WIDTH,
        },
        set_graphics_color,
        sprite::Sprite,
    },
    events::add_event,
    game::{Entity, Event, Shareable, State},
    load_sprite,
};
use alloc::{
    format,
    string::{String, ToString},
};
use cozy_chess::{Board, BoardBuilder, Color, File, Move, Piece, Rank, Square};
use engine::Eval;

pub const SQUARE_SIZE: usize = 20;
pub const BORDER_SIZE: usize = 4;

pub const BOARD_X: usize = 80;
pub const BOARD_Y: usize = 40;
const PROMOTION_X: usize = BOARD_X + 2 * SQUARE_SIZE;
const PROMOTION_Y: usize = (BOARD_Y - SQUARE_SIZE) / 2;
const ENGINE_EVAL_X: usize = BOARD_X + 8 * SQUARE_SIZE + BORDER_SIZE + 5;
const ENGINE_EVAL_Y: usize = BOARD_Y;
const ENGINE_THINKING_X: usize = ENGINE_EVAL_X;
const ENGINE_THINKING_Y: usize = ENGINE_EVAL_Y + SQUARE_SIZE * 2;

const CHESSBOARD: Sprite = load_sprite!("../sprites/chessboard.data", SQUARE_SIZE * 8);
const CHESSBOARD_BORDER: Sprite = load_sprite!(
    "../sprites/ChessBoardBorder.data",
    SQUARE_SIZE * 8 + BORDER_SIZE * 2
);
const PROMOTION_BACKGROUND: Sprite =
    load_sprite!("../sprites/PromotionBackground.data", SQUARE_SIZE * 4);

const W_PAWN: Sprite = load_sprite!("../sprites/WhitePawn.data", SQUARE_SIZE);
const W_ROOK: Sprite = load_sprite!("../sprites/WhiteRook.data", SQUARE_SIZE);
const W_KNIGHT: Sprite = load_sprite!("../sprites/WhiteKnight.data", SQUARE_SIZE);
const W_BISHOP: Sprite = load_sprite!("../sprites/WhiteBishop.data", SQUARE_SIZE);
const W_QUEEN: Sprite = load_sprite!("../sprites/WhiteQueen.data", SQUARE_SIZE);
const W_KING: Sprite = load_sprite!("../sprites/WhiteKing.data", SQUARE_SIZE);

const B_PAWN: Sprite = load_sprite!("../sprites/BlackPawn.data", SQUARE_SIZE);
const B_ROOK: Sprite = load_sprite!("../sprites/BlackRook.data", SQUARE_SIZE);
const B_KNIGHT: Sprite = load_sprite!("../sprites/BlackKnight.data", SQUARE_SIZE);
const B_BISHOP: Sprite = load_sprite!("../sprites/BlackBishop.data", SQUARE_SIZE);
const B_QUEEN: Sprite = load_sprite!("../sprites/BlackQueen.data", SQUARE_SIZE);
const B_KING: Sprite = load_sprite!("../sprites/BlackKing.data", SQUARE_SIZE);

const EMPTY_SQUARE: Sprite = load_sprite!("../sprites/EmptySquare.data", SQUARE_SIZE);
const PIECE_SELECTED: Sprite = load_sprite!("../sprites/PieceSelected.data", SQUARE_SIZE);
const PIECE_DESTINATION: Sprite = load_sprite!("../sprites/PieceDestination.data", SQUARE_SIZE);
const PIECE_CAPTURE: Sprite = load_sprite!("../sprites/PieceCapture.data", SQUARE_SIZE);

const KING_BLUSH: Sprite = load_sprite!("../sprites/KingBlush.data", SQUARE_SIZE);

const ENGINE_THINKING: Sprite = load_sprite!("../sprites/EngineThinking.data", 40);
const CHOOSE_WHITE_SPRITE: Sprite = load_sprite!("../sprites/White.data", 32);
const CHOOSE_BLACK_SPRITE: Sprite = load_sprite!("../sprites/Black.data", 32);

fn piece_sprite(piece: Piece, color: Color) -> &'static Sprite {
    match (color, piece) {
        (Color::White, Piece::Pawn) => &W_PAWN,
        (Color::White, Piece::Rook) => &W_ROOK,
        (Color::White, Piece::Knight) => &W_KNIGHT,
        (Color::White, Piece::Bishop) => &W_BISHOP,
        (Color::White, Piece::Queen) => &W_QUEEN,
        (Color::White, Piece::King) => &W_KING,
        (Color::Black, Piece::Pawn) => &B_PAWN,
        (Color::Black, Piece::Rook) => &B_ROOK,
        (Color::Black, Piece::Knight) => &B_KNIGHT,
        (Color::Black, Piece::Bishop) => &B_BISHOP,
        (Color::Black, Piece::Queen) => &B_QUEEN,
        (Color::Black, Piece::King) => &B_KING,
    }
}

fn is_mouse_click(event: &Event) -> bool {
    match event {
        Event::MouseInput(input) => input.left_button_down(),
        _ => false,
    }
}

fn to_xy(start_x: usize, start_y: usize, index: usize) -> (usize, usize) {
    (
        start_x + (index % 8) * SQUARE_SIZE,
        start_y + (7 - (index / 8)) * SQUARE_SIZE,
    )
}

fn for_each_move<F>(sq: Square, board: &Board, mut f: F)
where
    F: FnMut(cozy_chess::Move) -> bool,
{
    let bb = sq.bitboard();
    board.generate_moves_for(bb, |mvs| {
        for mv in mvs {
            if f(mv) {
                return true;
            }
        }
        false
    });
}

fn handle_square_selection(prev: Square, curr: Square, board: &Board) {
    for_each_move(prev, board, |mv| {
        // If user has clicked on a possible square to move to, play it
        match (curr == mv.to, mv.promotion.is_some()) {
            (true, false) => {
                add_event(Event::PlayMove(mv));
                true
            }
            (true, true) => {
                add_event(Event::DisplayPromotion(prev, curr));
                true
            }
            _ => false,
        }
    });
}

pub fn is_checkmate(board: &Board) -> bool {
    let mut checkmate = true;
    board.generate_moves(|_| {
        checkmate = false;
        true
    });
    checkmate
}

fn sigmoid(val: i32) -> i32 {
    let computed = ((val / 20).abs().min(127) - 127).pow(2) / 128;

    if val > 0 {
        255 - computed
    } else {
        computed
    }
}

pub struct ChessBoard {
    square_selected: Option<cozy_chess::Square>,
}

pub struct PromotionDisplayer {
    from: Square,
    to: Square,
    to_delete: bool,
}

pub struct Text {
    rect: Rectangle,
    text: String,
    color: Color256,
}

pub struct Button<E: Entity> {
    rect: Rectangle,
    entity: E,
    on_click: Event,
}

pub struct EngineEval {
    text: Text,
    curr_eval: Eval,
}

pub struct SpriteEntity {
    x: usize,
    y: usize,
    sprite: &'static Sprite,
}

pub struct ColorSelector {
    white_button: Button<SpriteEntity>,
    black_button: Button<SpriteEntity>,
}

pub struct DifficultySelector {
    difficulties: [Button<Text>; Self::NUM_DIFFICULTIES],
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            square_selected: None,
        }
    }

    fn draw_board(&self, shared: &Shareable) {
        for (i, square) in Square::ALL.iter().enumerate() {
            let square = if shared.should_flip() {
                square.flip_rank()
            } else {
                *square
            };
            let piece_sprite = match (shared.board.color_on(square), shared.board.piece_on(square))
            {
                (Some(color), Some(piece)) => piece_sprite(piece, color),
                _ => continue,
            };

            let (x, y) = to_xy(BOARD_X, BOARD_Y, i);

            draw_sprite(piece_sprite, x, y);
        }
    }

    fn draw_overlay_squares(&self, shared: &Shareable) {
        // King blush if in check :3
        if shared.board.checkers().len() != 0 {
            let stm = shared.board.side_to_move();
            let mut sq = shared.board.king(stm);
            if shared.should_flip() {
                sq = sq.flip_rank();
            }
            let (x, y) = to_xy(BOARD_X, BOARD_Y, sq as usize);
            draw_sprite(&KING_BLUSH, x, y);
        }

        if self.square_selected.is_none() {
            return;
        }

        // Selected square
        let square = self.square_selected.unwrap();
        let display_square = if shared.should_flip() {
            square.flip_rank()
        } else {
            square
        };
        let (x, y) = to_xy(BOARD_X, BOARD_Y, display_square as usize);

        draw_sprite(&PIECE_SELECTED, x, y);

        // Destination squares
        for_each_move(square, &shared.board, |mv| {
            let display_square = if shared.should_flip() {
                mv.to.flip_rank()
            } else {
                mv.to
            };
            let (x, y) = to_xy(BOARD_X, BOARD_Y, display_square as usize);

            let sprite = if shared.board.piece_on(mv.to).is_some() {
                &PIECE_CAPTURE
            } else {
                &PIECE_DESTINATION
            };
            draw_sprite(sprite, x, y);
            false
        });
    }
}

impl PromotionDisplayer {
    const PIECES: [cozy_chess::Piece; 4] =
        [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];

    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            to_delete: false,
        }
    }
}

impl Text {
    pub fn new<S>(rect: Rectangle, text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            rect,
            text: text.into(),
            color: Color256::WHITE,
        }
    }

    pub fn set_color(&mut self, color: Color256) {
        self.color = color;
    }
}

impl<E: Entity> Button<E> {
    pub fn new(rect: Rectangle, entity: E, on_click: Event) -> Self {
        Self {
            rect,
            entity,
            on_click,
        }
    }
}

impl Button<Text> {
    pub fn with_text<S: Into<String>>(rect: Rectangle, text: S, on_click: Event) -> Self {
        Self {
            rect,
            entity: Text::new(rect, text),
            on_click,
        }
    }

    pub fn set_color(&mut self, color: Color256) {
        self.entity.set_color(color);
    }
}

impl Button<SpriteEntity> {
    pub fn with_sprite(rect: Rectangle, sprite: &'static Sprite, on_click: Event) -> Self {
        Self {
            rect,
            entity: SpriteEntity::new(rect.x, rect.y, sprite),
            on_click,
        }
    }
}

impl EngineEval {
    pub fn new() -> Self {
        let mut s = Self {
            text: Text::new(
                Rectangle {
                    x: ENGINE_EVAL_X,
                    y: ENGINE_EVAL_Y,
                    width: 6 * 8,
                    height: 16,
                },
                Self::eval_to_string(Eval::NEUTRAL),
            ),
            curr_eval: Eval::NEUTRAL,
        };
        s.calculate_new_color();
        s
    }

    pub fn eval_to_string(eval: Eval) -> String {
        match eval {
            Eval::MateIn(x) => format!("M{}", x),
            Eval::MatedIn(x) => format!("M-{}", x),
            Eval::CentiPawn(x) => format!("{}", x),
        }
    }

    fn calculate_new_color(&mut self) {
        // Calculating color to show
        // red is losing, green is winning, yellow is neutral

        let val = self.curr_eval.value();

        let computed = sigmoid(val);

        let r = (255 - computed) as u8;
        let g = computed as u8;
        let b = 0;

        let color = Color256::new(r, g, b);

        self.text.set_color(color)
    }
}

impl SpriteEntity {
    pub fn new(x: usize, y: usize, sprite: &'static Sprite) -> Self {
        Self { x, y, sprite }
    }
}

impl ColorSelector {
    pub fn new() -> Self {
        const CHOOSE_WHITE: Rectangle = Rectangle {
            x: (WIDTH - 80) / 2,
            y: 64,
            width: 32,
            height: 32,
        };

        const CHOOSE_BLACK: Rectangle = Rectangle {
            x: (WIDTH + 80) / 2 - 32,
            y: 64,
            width: 32,
            height: 32,
        };

        let white_button = Button::with_sprite(
            CHOOSE_WHITE,
            &CHOOSE_WHITE_SPRITE,
            Event::SetPlayerColor(cozy_chess::Color::White),
        );
        let black_button = Button::with_sprite(
            CHOOSE_BLACK,
            &CHOOSE_BLACK_SPRITE,
            Event::SetPlayerColor(cozy_chess::Color::Black),
        );

        Self {
            white_button,
            black_button,
        }
    }
}

impl DifficultySelector {
    const NUM_DIFFICULTIES: usize = 7;

    pub fn new() -> Self {
        const Y: usize = 16;
        const BUTTON_SIZE: usize = 16;
        const PADDING: usize = 4;
        let start_x =
            (WIDTH - Self::NUM_DIFFICULTIES * BUTTON_SIZE - (Self::NUM_DIFFICULTIES - 1) * PADDING)
                / 2;

        let difficulties = array::from_fn(|i| {
            let text = (i + 1).to_string();
            let rect = Rectangle {
                x: start_x + i * BUTTON_SIZE + i * PADDING,
                y: Y,
                width: BUTTON_SIZE,
                height: BUTTON_SIZE,
            };
            Button::with_text(rect, text, Event::SetEngineDepth(i as u8 + 1))
        });

        Self { difficulties }
    }
}

impl Entity for ChessBoard {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if shared.state == State::GameOver {
            return;
        }

        if is_checkmate(&shared.board) {
            add_event(Event::EndGame);
            return;
        }

        if !is_mouse_click(event) {
            return;
        }

        let board_x = (shared.mouse_x as usize).wrapping_sub(BOARD_X) / SQUARE_SIZE;
        let board_y = (shared.mouse_y as usize).wrapping_sub(BOARD_Y) / SQUARE_SIZE;

        if board_x > 7 || board_y > 7 {
            self.square_selected = None;
            return;
        }

        // Valid square has been selected in the board

        let sq_index = board_x + 8 * (7 - board_y);
        let mut selected = Square::index(sq_index);

        if shared.should_flip() {
            selected = selected.flip_rank();
        }

        match self.square_selected {
            // Clicking on same square
            Some(sq) if sq == selected => self.square_selected = None,
            Some(sq) => {
                handle_square_selection(sq, selected, &shared.board);
                self.square_selected = None;
            }
            None => self.square_selected = Some(selected),
        }
    }

    fn draw(&self, shared: &Shareable) {
        draw_sprite(&CHESSBOARD, BOARD_X, BOARD_Y);
        draw_sprite(
            &CHESSBOARD_BORDER,
            BOARD_X - BORDER_SIZE,
            BOARD_Y - BORDER_SIZE,
        );

        self.draw_board(shared);

        self.draw_overlay_squares(shared);

        if shared.engine_thinking {
            draw_sprite(&ENGINE_THINKING, ENGINE_THINKING_X, ENGINE_THINKING_Y);
        }
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

impl Entity for PromotionDisplayer {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let promotion_x = (shared.mouse_x as usize).wrapping_sub(PROMOTION_X) / SQUARE_SIZE;
        let promotion_y = (shared.mouse_y as usize).wrapping_sub(PROMOTION_Y) / SQUARE_SIZE;

        if promotion_x > 3 || promotion_y > 0 {
            return;
        }

        let piece = PromotionDisplayer::PIECES[promotion_x];

        add_event(Event::PlayMove(Move {
            from: self.from,
            to: self.to,
            promotion: Some(piece),
        }));

        self.to_delete = true;
    }

    fn draw(&self, shared: &Shareable) {
        let color = shared.board.side_to_move();

        draw_sprite(&PROMOTION_BACKGROUND, PROMOTION_X, PROMOTION_Y);

        for (i, piece) in PromotionDisplayer::PIECES.into_iter().enumerate() {
            let sprite = piece_sprite(piece, color);
            draw_sprite(sprite, PROMOTION_X + i * SQUARE_SIZE, PROMOTION_Y);
        }
    }

    fn to_delete(&self, shared: &Shareable) -> bool {
        self.to_delete || !shared.in_promotion
    }
}

impl Entity for Text {
    fn handle_event(&mut self, _: &Event, _: &Shareable) {}

    fn draw(&self, _: &Shareable) {
        set_graphics_color(self.color);

        draw_shape(&self.rect);

        let width = self.text.len() * CHAR_WIDTH;

        let x = self.rect.x + (self.rect.width - width) / 2;
        let y = self.rect.y + (self.rect.height - CHAR_HEIGHT) / 2;

        draw_text(self.text.bytes(), x, y);
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

impl<E: Entity> Entity for Button<E> {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let point = (shared.mouse_x as usize, shared.mouse_y as usize);
        if contains_point(&self.rect, point) {
            add_event(self.on_click.clone());
        }
    }

    fn draw(&self, shared: &Shareable) {
        self.entity.draw(shared)
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

impl Entity for EngineEval {
    fn handle_event(&mut self, _: &Event, shared: &Shareable) {
        if shared.engine_eval == self.curr_eval {
            return;
        }

        self.curr_eval = shared.engine_eval;
        self.text.text = Self::eval_to_string(self.curr_eval);

        self.calculate_new_color();
    }

    fn draw(&self, shared: &Shareable) {
        self.text.draw(shared);
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

impl Entity for SpriteEntity {
    fn handle_event(&mut self, _: &Event, _: &Shareable) {}

    fn draw(&self, _: &Shareable) {
        draw_sprite(self.sprite, self.x, self.y)
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

impl Entity for ColorSelector {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        self.white_button.handle_event(event, shared);
        self.black_button.handle_event(event, shared);
    }

    fn draw(&self, shared: &Shareable) {
        self.white_button.draw(shared);
        self.black_button.draw(shared);
        let rect = match shared.user_color {
            cozy_chess::Color::White => self.white_button.rect,
            cozy_chess::Color::Black => self.black_button.rect,
        };

        set_graphics_color(Color256::LIGHT_BLUE);
        draw_shape(&rect);
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

impl Entity for DifficultySelector {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        for (i, button) in self.difficulties.iter_mut().enumerate() {
            button.handle_event(event, shared);
            if (i as u8 + 1) == shared.engine_depth {
                button.set_color(Color256::LIGHT_BLUE);
            } else {
                button.set_color(Color256::WHITE);
            }
        }
    }

    fn draw(&self, shared: &Shareable) {
        for button in self.difficulties.iter() {
            button.draw(shared)
        }
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}

#[test_case]
fn test_sigmoid() {
    assert_eq!(sigmoid(0), 126);

    assert_eq!(sigmoid(100000000), 127);
    assert_eq!(sigmoid(-100000000), 0);
}
