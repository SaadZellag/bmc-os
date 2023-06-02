



pub(crate) trait CRELU<I, O, const N: usize> {
    fn crelu(&self, output_scale: I) -> [O; N];
}

macro_rules! impl_celu {
    ($input:ty, $output:ty, $zero:expr, $min:expr, $max:expr) => {
        impl<const N: usize> CRELU<$input, $output, N> for [$input; N] {
            fn crelu(&self, output_scale: $input) -> [$output; N] {
                let mut result = [$zero; N];
                for i in 0..N {
                    result[i] = (self[i] / output_scale).clamp($min, $max) as $output;
                }
                result
            }
        }
    };
}

impl_celu!(i16, i8, 0, 0, 127);
impl_celu!(i32, i8, 0, 0, 127);
