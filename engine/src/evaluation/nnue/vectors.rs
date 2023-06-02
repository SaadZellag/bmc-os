// https://doc.rust-lang.org/core/arch/index.html#examples

macro_rules! gen_functions {
    ($fn_name:ident, $fn_avx2:ident, $pub_fn_name:ident, $input:ty, $output:ty) => {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        pub fn $pub_fn_name<const N: usize>(a: &[$input; N], b: &[$input; N]) -> $output {
            unsafe { $fn_avx2(a, b) }
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        pub fn $pub_fn_name<const N: usize>(a: &[$input; N], b: &[$input; N]) -> $output {
            unsafe { $fn_avx2(a, b) }
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        #[target_feature(enable = "avx2")]
        unsafe fn $fn_avx2<const N: usize>(a: &[$input; N], b: &[$input; N]) -> $output {
            $fn_name(a, b)
        }
    };
}

gen_functions!(vadd, vadd_avx2, fast_vadd, i16, [i16; N]);
gen_functions!(vsub, vsub_avx2, fast_vsub, i16, [i16; N]);
gen_functions!(vdot, vdot_avx2, fast_vdot, i8, i32);

fn vadd<const N: usize>(a: &[i16; N], b: &[i16; N]) -> [i16; N] {
    let mut result = [0; N];
    for i in 0..N {
        result[i] = a[i] + b[i];
    }
    result
}

fn vsub<const N: usize>(a: &[i16; N], b: &[i16; N]) -> [i16; N] {
    let mut result = [0; N];
    for i in 0..N {
        result[i] = a[i] - b[i];
    }
    result
}

fn vdot<const N: usize>(a: &[i8; N], b: &[i8; N]) -> i32 {
    let mut result = 0;
    for i in 0..N {
        result += a[i] as i32 * b[i] as i32;
    }
    result
}
