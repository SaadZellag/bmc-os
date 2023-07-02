use crate::{
    display::{
        graphics::{contains_point, draw_sprite, Rectangle},
        sprite::Sprite,
    },
    events::add_event,
    game::{Entity, Event, Shareable},
    load_sprite,
};
use cozy_chess::{Board, BoardBuilder, Color, File, Move, Piece, Rank, Square};

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
const PIECE_CAPTURE: Sprite = load_sprite!("../sprites/PieceCapture.data", SQUARE_SIZE);

const KING_BLUSH: Sprite = load_sprite!("../sprites/KingBlush.data", SQUARE_SIZE);

fn is_mouse_click(event: &Event) -> bool {
    match event {
        Event::MouseInput(input) => input.left_button_down(),
        _ => false,
    }
}

fn to_xy(index: usize) -> (usize, usize) {
    (
        BOARD_X + (index % 8) * SQUARE_SIZE,
        BOARD_Y + (7 - (index / 8)) * SQUARE_SIZE,
    )
}

fn for_each_move<F>(sq: Square, board: &Board, mut f: F)
where
    F: FnMut(cozy_chess::Move),
{
    let bb = sq.bitboard();
    board.generate_moves_for(bb, |mvs| {
        for mv in mvs {
            f(mv);
        }
        false
    });
}
pub struct ChessBoard {
    square_selected: Option<cozy_chess::Square>,
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
                _ => continue,
            };

            let (x, y) = to_xy(i);

            draw_sprite(piece_sprite, x, y);
        }
    }

    fn draw_overlay_squares(&self, shared: &Shareable) {
        // King blush if in check :3
        if shared.board.checkers().len() != 0 {
            let stm = shared.board.side_to_move();
            let sq = shared.board.king(stm);
            let (x, y) = to_xy(sq as usize);
            draw_sprite(&KING_BLUSH, x, y);
        }

        if self.square_selected.is_none() {
            return;
        }

        // Selected square
        let square = self.square_selected.unwrap();
        let (x, y) = to_xy(square as usize);

        draw_sprite(&PIECE_SELECTED, x, y);

        // Destination squares
        for_each_move(square, &shared.board, |mv| {
            let (x, y) = to_xy(mv.to as usize);

            let sprite = if shared.board.piece_on(mv.to).is_some() {
                &PIECE_CAPTURE
            } else {
                &PIECE_DESTINATION
            };
            draw_sprite(sprite, x, y);
        });
    }
}

impl Entity for ChessBoard {
    fn handle_event(&mut self, event: &Event, shared: &Shareable) {
        if !is_mouse_click(event) {
            return;
        }

        let board_x = (shared.mouse_x as usize).wrapping_sub(BOARD_X) / SQUARE_SIZE;
        let board_y = (shared.mouse_y as usize).wrapping_sub(BOARD_Y) / SQUARE_SIZE;

        if board_x > 7 || board_y > 7 {
            self.square_selected = None;
            return;
        }

        // Valid square selected in the board

        let sq_index = board_x + 8 * (7 - board_y);
        let selected = Square::index(sq_index);

        match self.square_selected {
            // Clicking on same square
            Some(sq) if sq == selected => self.square_selected = None,
            Some(sq) => {
                for_each_move(sq, &shared.board, |mv| {
                    // If user has clicked on a possible square to move to, play it
                    if selected == mv.to {
                        add_event(Event::PlayMove(mv))
                    }
                });
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
