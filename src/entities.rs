use crate::{
    display::{
        color::Color256,
        graphics::{
            contains_point, draw_shape, draw_sprite, draw_text, Rectangle, CHAR_HEIGHT, CHAR_WIDTH,
        },
        set_graphics_color,
        sprite::Sprite,
    },
    events::add_event,
    game::{Entity, Event, Shareable, State},
    load_sprite,
};
use alloc::string::String;
use cozy_chess::{Board, BoardBuilder, Color, File, Move, Piece, Rank, Square};

pub const SQUARE_SIZE: usize = 20;
pub const BORDER_SIZE: usize = 4;

pub const BOARD_X: usize = 80;
pub const BOARD_Y: usize = 40;
const PROMOTION_X: usize = BOARD_X + 2 * SQUARE_SIZE;
const PROMOTION_Y: usize = (BOARD_Y - SQUARE_SIZE) / 2;

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

const TEXT: Sprite = load_sprite!("../sprites/Text.data", 16 * 16);

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

fn is_checkmate(board: &Board) -> bool {
    let mut checkmate = true;
    board.generate_moves(|_| {
        checkmate = false;
        true
    });
    checkmate
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

pub struct Button {
    text: Text,
    on_click: Event,
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            square_selected: None,
        }
    }

    fn draw_board(&self, shared: &Shareable) {
        for (i, &square) in Square::ALL.iter().enumerate() {
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
            let sq = shared.board.king(stm);
            let (x, y) = to_xy(BOARD_X, BOARD_Y, sq as usize);
            draw_sprite(&KING_BLUSH, x, y);
        }

        if self.square_selected.is_none() {
            return;
        }

        // Selected square
        let square = self.square_selected.unwrap();
        let (x, y) = to_xy(BOARD_X, BOARD_Y, square as usize);

        draw_sprite(&PIECE_SELECTED, x, y);

        // Destination squares
        for_each_move(square, &shared.board, |mv| {
            let (x, y) = to_xy(BOARD_X, BOARD_Y, mv.to as usize);

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

impl Button {
    pub fn new(rect: Rectangle, text: &'static str, on_click: Event) -> Self {
        Self {
            text: Text::new(rect, text),
            on_click,
        }
    }

    pub fn set_color(&mut self, color: Color256) {
        self.text.set_color(color);
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
        let selected = Square::index(sq_index);

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

impl Entity for Button {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let point = (shared.mouse_x as usize, shared.mouse_y as usize);
        if contains_point(&self.text.rect, point) {
            add_event(self.on_click.clone());
        }
    }

    fn draw(&self, shared: &Shareable) {
        self.text.draw(shared)
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
