use crate::{
    display::{
        graphics::{contains_point, draw_sprite, Rectangle},
        sprite::Sprite,
    },
    game::{Entity, Event, Shareable},
    load_sprite,
};
use cozy_chess::{Board, BoardBuilder, Color, File, Piece, Rank, Square};

const SQUARE_SIZE: usize = 20;
const BORDER_SIZE: usize = 4;
const BOARD_X: usize = 80;
const BOARD_Y: usize = 40;

const CHESSBOARD: Sprite = load_sprite!("../sprites/chessboard.data", SQUARE_SIZE * 8);
const CHESSBOARD_BORDER: Sprite = load_sprite!(
    "../sprites/ChessBoardBorder.data",
    SQUARE_SIZE * 8 + BORDER_SIZE * 2
);

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

fn is_mouse_click(event: &Event) -> bool {
    match event {
        Event::MouseInput(input) => input.left_button_down(),
        _ => false,
    }
}

#[derive(Debug, Clone, Copy)]
enum SquareStatus {
    None,
    PieceSelected,
    PieceDestination,
}

pub struct ChessBoard {
    statuses: [SquareStatus; 64],
}

impl ChessBoard {
    pub fn new() -> Self {
        Self {
            statuses: [SquareStatus::None; 64],
        }
    }
}

impl Entity for ChessBoard {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let board_x = (shared.mouse_x as usize - BOARD_X) / SQUARE_SIZE;
        let board_y = (shared.mouse_y as usize - BOARD_Y) / SQUARE_SIZE;

        if board_x > 7 || board_y > 7 {
            return;
        }

        let sq_index = board_x + 8 * (7 - board_y);

        for status in self.statuses.iter_mut() {
            *status = SquareStatus::None;
        }

        self.statuses[sq_index] = SquareStatus::PieceSelected;
    }

    fn draw(&self, shared: &Shareable) {
        draw_sprite(&CHESSBOARD, BOARD_X, BOARD_Y);
        draw_sprite(
            &CHESSBOARD_BORDER,
            BOARD_X - BORDER_SIZE,
            BOARD_Y - BORDER_SIZE,
        );

        for (i, &square) in Square::ALL.iter().enumerate() {
            let x = i % 8;
            let y = 7 - (i / 8);

            let piece_sprite = match (shared.board.color_on(square), shared.board.piece_on(square))
            {
                (Some(Color::White), Some(Piece::Pawn)) => &W_PAWN,
                (Some(Color::White), Some(Piece::Rook)) => &W_ROOK,
                (Some(Color::White), Some(Piece::Knight)) => &W_KNIGHT,
                (Some(Color::White), Some(Piece::Bishop)) => &W_BISHOP,
                (Some(Color::White), Some(Piece::Queen)) => &W_QUEEN,
                (Some(Color::White), Some(Piece::King)) => &W_KING,
                (Some(Color::Black), Some(Piece::Pawn)) => &B_PAWN,
                (Some(Color::Black), Some(Piece::Rook)) => &B_ROOK,
                (Some(Color::Black), Some(Piece::Knight)) => &B_KNIGHT,
                (Some(Color::Black), Some(Piece::Bishop)) => &B_BISHOP,
                (Some(Color::Black), Some(Piece::Queen)) => &B_QUEEN,
                (Some(Color::Black), Some(Piece::King)) => &B_KING,
                _ => &EMPTY_SQUARE,
            };

            let overlay_sprite = match self.statuses[i] {
                SquareStatus::None => &EMPTY_SQUARE,
                SquareStatus::PieceSelected => &PIECE_SELECTED,
                SquareStatus::PieceDestination => &PIECE_DESTINATION,
            };

            draw_sprite(
                piece_sprite,
                BOARD_X + x * SQUARE_SIZE,
                BOARD_Y + y * SQUARE_SIZE,
            );

            draw_sprite(
                overlay_sprite,
                BOARD_X + x * SQUARE_SIZE,
                BOARD_Y + y * SQUARE_SIZE,
            );
        }
    }

    fn to_delete(&self, _: &Shareable) -> bool {
        false
    }
}
