use core::{iter, marker::PhantomData};

use cozy_chess::{Board, Color, Piece, Square};

use crate::evaluation::nnue::features::{FeatureSet, HalfKP, SPC};

pub struct BoardFeatures<F: FeatureSet> {
    _marker: PhantomData<F>,
}
pub trait SquareUtils {
    fn flip(self) -> Self;
}

impl BoardFeatures<SPC> {
    pub fn features(board: &Board) -> impl Iterator<Item = (usize, usize)> + '_ {
        Piece::ALL
            .iter()
            .flat_map(|&p| iter::repeat(p).zip(board.pieces(p)))
            .map(|(p, sq)| {
                let color = board.color_on(sq).expect("Corrupted board");
                let white_index = SPC::white_feature_index(sq, p, color);
                let black_index = SPC::black_feature_index(sq, p, color);

                (white_index, black_index)
            })
    }
}

impl BoardFeatures<HalfKP> {
    pub fn features(board: &Board) -> impl Iterator<Item = (usize, usize)> + '_ {
        let white_king_sq = board
            .colored_pieces(Color::White, Piece::King)
            .next_square()
            .unwrap();
        let black_king_sq = board
            .colored_pieces(Color::Black, Piece::King)
            .next_square()
            .unwrap();

        HalfKP::HALFKP_PIECES
            .iter()
            .flat_map(|&p| iter::repeat(p).zip(board.pieces(p)))
            .map(move |(p, sq)| {
                let color = board.color_on(sq).expect("Corrupted board");
                let white_index = HalfKP::white_feature_index(white_king_sq, sq, p, color);
                let black_index = HalfKP::black_feature_index(black_king_sq, sq, p, color);

                (white_index, black_index)
            })
    }
}

impl SquareUtils for Square {
    fn flip(self) -> Self {
        self.flip_file() // TODO: Note sure of this
    }
}
