#![cfg_attr(feature = "unstable", feature(test))]

extern crate quicksort;
extern crate rand;
extern crate rdxsort;

#[cfg(feature = "unstable")]
mod unstable {
    extern crate test;
    use self::test::Bencher;

    use quicksort::quicksort_by;

    use rand::{Rand, Rng, XorShiftRng};

    use rdxsort::*;

    static N_SMALL:  usize = 1_000;
    static N_MEDIUM: usize = 10_000;
    static N_LARGE:  usize = 100_000;

    fn bench_exe<T, F>(b: &mut Bencher, data: Vec<T>, f: F) where T: Clone, F: Fn(Vec<T>) {
        let _ = b.iter(|| {
            let data2 = data.clone();
            f(data2);
        });
    }

    fn bench_generic<T, F>(b: &mut Bencher, f: F, n: usize)
        where T: Clone + PartialOrd + Rand,
              F: Fn(Vec<T>)
    {
        // generate data
        let mut rng = XorShiftRng::new_unseeded();
        let data: Vec<T> = rng.gen_iter::<T>().take(n).collect();

        // run benchmark
        bench_exe(b, data, f);
    }

    fn bench_quicksort_generic<T>(b: &mut Bencher, n: usize)
        where T: Clone + PartialOrd + Rand,
              Vec<T>: RdxSort
    {
        bench_generic(b, |data| {
            let mut data = data;
            quicksort_by(data.as_mut_slice(), |a: &T, b: &T| a.partial_cmp(b).unwrap());
        }, n);
    }

    fn bench_std_generic<T>(b: &mut Bencher, n: usize)
        where T: Clone + PartialOrd + Rand,
              Vec<T>: RdxSort
    {
        bench_generic(b, |data| {
            let mut data = data;
            data.sort_by(|a: &T, b: &T| a.partial_cmp(b).unwrap());
        }, n);
    }

    fn bench_rdxsort_generic<T>(b: &mut Bencher, n: usize)
        where T: Clone + PartialOrd + Rand,
              Vec<T>: RdxSort
    {
        bench_generic(b, |data| {
            let mut data = data;
            data.rdxsort();
        }, n);
    }

    macro_rules! bench_type {
        ($t:ty, [
            $fn_small_quicksort:ident,
            $fn_small_rdxsort:ident,
            $fn_small_std:ident,
            $fn_medium_quicksort:ident,
            $fn_medium_rdxsort:ident,
            $fn_medium_std:ident,
            $fn_large_quicksort:ident,
            $fn_large_rdxsort:ident,
            $fn_large_std:ident
        ]) => {
            #[bench]
            fn $fn_small_quicksort(b: &mut Bencher) {
                bench_quicksort_generic::<$t>(b, N_SMALL);
            }

            #[bench]
            fn $fn_small_rdxsort(b: &mut Bencher) {
                bench_rdxsort_generic::<$t>(b, N_SMALL);
            }

            #[bench]
            fn $fn_small_std(b: &mut Bencher) {
                bench_std_generic::<$t>(b, N_SMALL);
            }

            #[bench]
            fn $fn_medium_quicksort(b: &mut Bencher) {
                bench_quicksort_generic::<$t>(b, N_MEDIUM);
            }

            #[bench]
            fn $fn_medium_rdxsort(b: &mut Bencher) {
                bench_rdxsort_generic::<$t>(b, N_MEDIUM);
            }

            #[bench]
            fn $fn_medium_std(b: &mut Bencher) {
                bench_std_generic::<$t>(b, N_MEDIUM);
            }

            #[bench]
            fn $fn_large_quicksort(b: &mut Bencher) {
                bench_quicksort_generic::<$t>(b, N_LARGE);
            }

            #[bench]
            fn $fn_large_rdxsort(b: &mut Bencher) {
                bench_rdxsort_generic::<$t>(b, N_LARGE);
            }

            #[bench]
            fn $fn_large_std(b: &mut Bencher) {
                bench_std_generic::<$t>(b, N_LARGE);
            }
        };
    }

    bench_type!(bool, [
        bench_small_bool_quicksort,
        bench_small_bool_rdxsort,
        bench_small_bool_std,
        bench_medium_bool_quicksort,
        bench_medium_bool_rdxsort,
        bench_medium_bool_std,
        bench_large_bool_quicksort,
        bench_large_bool_rdxsort,
        bench_large_bool_std
    ]);

    bench_type!(char, [
        bench_small_char_quicksort,
        bench_small_char_rdxsort,
        bench_small_char_std,
        bench_medium_char_quicksort,
        bench_medium_char_rdxsort,
        bench_medium_char_std,
        bench_large_char_quicksort,
        bench_large_char_rdxsort,
        bench_large_char_std
    ]);

    bench_type!(f32, [
        bench_small_f32_quicksort,
        bench_small_f32_rdxsort,
        bench_small_f32_std,
        bench_medium_f32_quicksort,
        bench_medium_f32_rdxsort,
        bench_medium_f32_std,
        bench_large_f32_quicksort,
        bench_large_f32_rdxsort,
        bench_large_f32_std
    ]);

    bench_type!(f64, [
        bench_small_f64_quicksort,
        bench_small_f64_rdxsort,
        bench_small_f64_std,
        bench_medium_f64_quicksort,
        bench_medium_f64_rdxsort,
        bench_medium_f64_std,
        bench_large_f64_quicksort,
        bench_large_f64_rdxsort,
        bench_large_f64_std
    ]);

    bench_type!(i8, [
        bench_small_i8_quicksort,
        bench_small_i8_rdxsort,
        bench_small_i8_std,
        bench_medium_i8_quicksort,
        bench_medium_i8_rdxsort,
        bench_medium_i8_std,
        bench_large_i8_quicksort,
        bench_large_i8_rdxsort,
        bench_large_i8_std
    ]);

    bench_type!(i16, [
        bench_small_i16_quicksort,
        bench_small_i16_rdxsort,
        bench_small_i16_std,
        bench_medium_i16_quicksort,
        bench_medium_i16_rdxsort,
        bench_medium_i16_std,
        bench_large_i16_quicksort,
        bench_large_i16_rdxsort,
        bench_large_i16_std
    ]);

    bench_type!(i32, [
        bench_small_i32_quicksort,
        bench_small_i32_rdxsort,
        bench_small_i32_std,
        bench_medium_i32_quicksort,
        bench_medium_i32_rdxsort,
        bench_medium_i32_std,
        bench_large_i32_quicksort,
        bench_large_i32_rdxsort,
        bench_large_i32_std
    ]);

    bench_type!(i64, [
        bench_small_i64_quicksort,
        bench_small_i64_rdxsort,
        bench_small_i64_std,
        bench_medium_i64_quicksort,
        bench_medium_i64_rdxsort,
        bench_medium_i64_std,
        bench_large_i64_quicksort,
        bench_large_i64_rdxsort,
        bench_large_i64_std
    ]);

    bench_type!(u8, [
        bench_small_u8_quicksort,
        bench_small_u8_rdxsort,
        bench_small_u8_std,
        bench_medium_u8_quicksort,
        bench_medium_u8_rdxsort,
        bench_medium_u8_std,
        bench_large_u8_quicksort,
        bench_large_u8_rdxsort,
        bench_large_u8_std
    ]);

    bench_type!(u16, [
        bench_small_u16_quicksort,
        bench_small_u16_rdxsort,
        bench_small_u16_std,
        bench_medium_u16_quicksort,
        bench_medium_u16_rdxsort,
        bench_medium_u16_std,
        bench_large_u16_quicksort,
        bench_large_u16_rdxsort,
        bench_large_u16_std
    ]);

    bench_type!(u32, [
        bench_small_u32_quicksort,
        bench_small_u32_rdxsort,
        bench_small_u32_std,
        bench_medium_u32_quicksort,
        bench_medium_u32_rdxsort,
        bench_medium_u32_std,
        bench_large_u32_quicksort,
        bench_large_u32_rdxsort,
        bench_large_u32_std
    ]);

    bench_type!(u64, [
        bench_small_u64_quicksort,
        bench_small_u64_rdxsort,
        bench_small_u64_std,
        bench_medium_u64_quicksort,
        bench_medium_u64_rdxsort,
        bench_medium_u64_std,
        bench_large_u64_quicksort,
        bench_large_u64_rdxsort,
        bench_large_u64_std
    ]);
}
