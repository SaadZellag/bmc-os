#[macro_export]
macro_rules! for_loop {
    ($index: ident $to_loop:expr; $inner_loop:expr) => {
        for $index in $to_loop {
            $inner_loop
        }
    };

    ($index: ident $to_loop:expr, $($o_index: ident $o_to_loop:expr),*; $inner_loop:expr) => {
        for $index in $to_loop {
            for_loop!($($o_index $o_to_loop),*; $inner_loop)
        }
    };
}

pub mod chessutils;
pub mod tablesize;
