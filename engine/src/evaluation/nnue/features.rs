use cozy_chess::{Color, Piece, Square};

use crate::utils::chessutils::SquareUtils;

pub trait FeatureSet {
    const TOTAL_FEATURES: usize;
    const FEATURES_PER_SIDE: usize;
    type Features;
    type Args;

    fn white_feature_index(args: Self::Args) -> usize;

    fn black_feature_index(args: Self::Args) -> usize;
}

pub struct SPC {}
pub struct HalfKP {}

impl HalfKP {
    pub const HALFKP_PIECES: [Piece; 5] = [
        Piece::Pawn,
        Piece::Knight,
        Piece::Bishop,
        Piece::Rook,
        Piece::Queen,
    ];
}

impl FeatureSet for SPC {
    const TOTAL_FEATURES: usize = 64 * 6 * 2;
    const FEATURES_PER_SIDE: usize = 32;

    type Features = [[u16; Self::FEATURES_PER_SIDE]; 2];
    type Args = (Square, Piece, Color);

    fn white_feature_index((piece_sq, piece, color): Self::Args) -> usize {
        piece_sq as usize + 64 * (piece as usize + 6 * color as usize)
    }

    fn black_feature_index((piece_sq, piece, color): Self::Args) -> usize {
        <Self as FeatureSet>::white_feature_index((piece_sq.flip(), piece, !color))
    }
}

impl FeatureSet for HalfKP {
    const TOTAL_FEATURES: usize = 64 * 64 * 5 * 2;
    const FEATURES_PER_SIDE: usize = 30;

    type Features = [[u16; Self::FEATURES_PER_SIDE]; 2];

    type Args = (Square, Square, Piece, Color);

    fn white_feature_index((king_sq, piece_sq, piece, color): Self::Args) -> usize {
        let p_idx = piece as usize * 2 + color as usize;
        piece_sq as usize + (p_idx + king_sq as usize * 10) * 64
    }

    fn black_feature_index((king_sq, piece_sq, piece, color): Self::Args) -> usize {
        <Self as FeatureSet>::white_feature_index((king_sq.flip(), piece_sq.flip(), piece, !color))
    }
}

macro_rules! unpack {
    ($struct: ty, $trait: ty, $($fn_name:ident($($arg:ident: $arg_type:ty),*) -> $return_type: ty),*) => {
        impl $struct {
            $(pub fn $fn_name($($arg: $arg_type),*) -> $return_type {
                <Self as $trait>::$fn_name(($($arg),*))
            })*
        }
    };
}

unpack!(SPC, FeatureSet,
    white_feature_index(piece_sq: Square, piece: Piece, color: Color) -> usize,
    black_feature_index(piece_sq: Square, piece: Piece, color: Color) -> usize
);

unpack!(HalfKP, FeatureSet,
    white_feature_index(king_sq: Square, piece_sq: Square, piece: Piece, color: Color) -> usize,
    black_feature_index(king_sq: Square, piece_sq: Square, piece: Piece, color: Color) -> usize
);

#[test]
fn test_spc() {
    use cozy_chess::{ALL_COLORS, ALL_PIECES, ALL_SQUARES};
    let mut result = [0; SPC::TOTAL_FEATURES];
    for_loop!(square ALL_SQUARES, piece ALL_PIECES, color ALL_COLORS; {
        let index = SPC::white_feature_index(square, piece, color);
                result[index] += 1;
    });

    assert_eq!(result, [1; SPC::TOTAL_FEATURES])
}

#[test]
fn test_halfkp() {
    use cozy_chess::{ALL_COLORS, ALL_SQUARES};
    let mut result = [0; HalfKP::TOTAL_FEATURES];

    for_loop!(king_sq ALL_SQUARES, square ALL_SQUARES, piece HalfKP::HALFKP_PIECES, color ALL_COLORS; {
        let index = HalfKP::white_feature_index(king_sq, square, piece, color);
        result[index] += 1;
    });

    assert_eq!(result, [1; HalfKP::TOTAL_FEATURES])
}
