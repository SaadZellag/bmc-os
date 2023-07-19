pub mod features;
pub mod layer;
pub mod nnue_conf;
pub mod utils;
mod vectors;

// https://github.com/glinscott/nnue-pytorch/blob/master/docs/nnue.md

use core::ops::{Index, IndexMut};

use cozy_chess::{Board, Color, Move, Piece, Square};

use crate::{
    evaluation::nnue::{
        layer::{FeatureLayer, Layer},
        nnue_conf::{CurrentFeatures, L1, L2, NUM_FEATURES},
        utils::CRELU,
        vectors::{fast_vadd, fast_vsub},
    },
    utils::chessutils::BoardFeatures,
    Eval,
};

#[repr(C)]
pub struct NNUE {
    ft: FeatureLayer<NUM_FEATURES, L1>,
    layer_1: Layer<{ L1 * 2 }, L2>,
    output: Layer<L2, 1>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(align(64))]
pub struct NNUEAccumulator {
    v: [[i16; L1]; 2],
}

impl Index<Color> for NNUEAccumulator {
    type Output = [i16; L1];

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self.v[0],
            Color::Black => &self.v[1],
        }
    }
}

impl IndexMut<Color> for NNUEAccumulator {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self.v[0],
            Color::Black => &mut self.v[1],
        }
    }
}

impl NNUE {
    pub fn eval(&self, acc: &NNUEAccumulator, stm: Color) -> Eval {
        let mut input = [0; { L1 * 2 }];

        for i in 0..L1 {
            input[i] = acc[stm][i].into();
            input[L1 + i] = acc[!stm][i].into();
        }

        let layer_1_out = self.layer_1.activate(&input.crelu(1));
        // let layer_2_out = self.layer_2.activate(&layer_1_out.crelu(64));
        let output = self.output.activate(&layer_1_out.crelu(64));

        Eval::CentiPawn(output[0] / 8)
    }
}

impl NNUEAccumulator {
    pub fn new(board: &Board, nnue: &NNUE) -> Self {
        let mut result = Self::empty(nnue);

        let features = BoardFeatures::<CurrentFeatures>::features(board);

        for (white, black) in features {
            result.add_feature(white, Color::White, nnue);
            result.add_feature(black, Color::Black, nnue);
        }

        result
    }

    pub fn empty(nnue: &NNUE) -> Self {
        let mut result = Self { v: [[0; L1]; 2] };

        for i in 0..L1 {
            result[Color::White][i] = nnue.ft.bias[i].into();
            result[Color::Black][i] = nnue.ft.bias[i].into();
        }

        result
    }

    pub fn add_feature(&mut self, index: usize, color: Color, nnue: &NNUE) {
        self[color] = fast_vadd(&self[color], &nnue.ft.weights[index]);
        // for i in 0..L1 {
        //     self[color][i] += nnue.ft.weights[index][i];
        // }
    }

    pub fn remove_feature(&mut self, index: usize, color: Color, nnue: &NNUE) {
        self[color] = fast_vsub(&self[color], &nnue.ft.weights[index]);
        // for i in 0..L1 {
        //     self[color][i] -= nnue.ft.weights[index][i];
        // }
    }

    pub fn update(
        &self,
        nnue: &NNUE,
        initial_board: &Board,
        final_board: &Board,
        mv: Move,
    ) -> Self {
        // debug_assert_eq!(&initial_board.make_move_new(mv), final_board);
        let piece_moving = initial_board
            .piece_on(mv.from)
            .expect("Invalid move for board");

        // Since our feature set may be halfkp, any move by the king may
        // reset every single feature, it only becomes problematic in the endgame
        // where the king moves a lot, but the number of pieces aren't many
        if piece_moving == Piece::King {
            return Self::new(final_board, nnue);
        }

        let mut result = self.clone();

        let from = mv.from;
        let to = mv.to;
        let stm = initial_board.side_to_move();

        let piece_captured = initial_board.piece_on(to);

        // Check if there is capture
        if let Some(captured) = piece_captured {
            let captured_color = initial_board.color_on(to).unwrap(); // Should always unwrap
            let white_index = CurrentFeatures::white_feature_index(to, captured, captured_color);
            let black_index = CurrentFeatures::black_feature_index(to, captured, captured_color);

            result.remove_feature(white_index, Color::White, nnue);
            result.remove_feature(black_index, Color::Black, nnue);
        }

        // Removing from square
        let white_index = CurrentFeatures::white_feature_index(from, piece_moving, stm);
        let black_index = CurrentFeatures::black_feature_index(from, piece_moving, stm);

        result.remove_feature(white_index, Color::White, nnue);
        result.remove_feature(black_index, Color::Black, nnue);

        // Promotion
        let piece_at_destination = match mv.promotion {
            Some(piece) => piece,
            None => piece_moving,
        };

        // Adding to square
        let white_index = CurrentFeatures::white_feature_index(to, piece_at_destination, stm);
        let black_index = CurrentFeatures::black_feature_index(to, piece_at_destination, stm);

        result.add_feature(white_index, Color::White, nnue);
        result.add_feature(black_index, Color::Black, nnue);

        // En Passant
        // If pawn moved in diagonal and it ate nothing
        let en_passant =
            piece_moving == Piece::Pawn && from.file() != to.file() && piece_captured.is_none();

        if en_passant {
            let square = Square::new(to.file(), from.rank());
            // Removing pawn
            // Adding to square
            let white_index = CurrentFeatures::white_feature_index(square, Piece::Pawn, !stm);
            let black_index = CurrentFeatures::black_feature_index(square, Piece::Pawn, !stm);

            result.remove_feature(white_index, Color::White, nnue);
            result.remove_feature(black_index, Color::Black, nnue);
        }

        result
    }
}

impl NNUE {
    pub const fn new() -> Self {
        const DATA: &'static [u8; core::mem::size_of::<NNUE>()] = include_bytes!("model.nnue");

        // Since the byte layout of the file is the same as the nnue, transmuting it is fine
        unsafe { core::mem::transmute(*DATA) }
    }
}

#[test]
fn test_acc_update() {
    use crate::engine::EVALUATOR;
    use crate::utils::positiongen::PositionGenerator;
    use cozy_chess::MoveGen;
    for board in PositionGenerator::new().take(1000) {
        let acc = NNUEAccumulator::new(&board, &EVALUATOR);

        for mv in MoveGen::new_legal(&board) {
            let mut new_board = board.clone();
            new_board.play_unchecked(mv);
            let new_acc = acc.update(&EVALUATOR, &board, &new_board, mv);

            assert_eq!(
                new_acc,
                NNUEAccumulator::new(&new_board, &EVALUATOR),
                "{} with {} played doesn't match",
                board,
                mv
            );
        }
    }
}
