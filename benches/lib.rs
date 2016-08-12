#![cfg_attr(feature = "unstable", feature(test))]

extern crate rand;
extern crate rdxsort;

#[cfg(feature = "unstable")]
mod unstable {
    extern crate test;
    use self::test::Bencher;

    use rand::XorShiftRng;
    use rand::distributions::{IndependentSample, Range};
    use rand::distributions::range::SampleRange;

    use rdxsort::*;

    static N_SMALL:  usize = 1_000;
    static N_MEDIUM: usize = 10_000;
    static N_LARGE:  usize = 100_000;

    fn bench_generic<T, F>(b: &mut Bencher, vmin: T, vmax: T, f: F, n: usize)
        where T: Clone + PartialOrd + SampleRange,
              F: Fn(Vec<T>)
    {
        // generate data
        let r = Range::new(vmin.clone(), vmax.clone());
        let mut rng = XorShiftRng::new_unseeded();
        let mut data = Vec::with_capacity(n);
        for _ in (0 as usize)..n {
            data.push(r.ind_sample(&mut rng));
        }

        // run benchmark
        let _ = b.iter(|| {
            let data2 = data.clone();
            f(data2);
        });
    }

    fn bench_std_generic<T>(b: &mut Bencher, vmin: T, vmax: T, n: usize)
        where T: Clone + PartialOrd + SampleRange,
              Vec<T>: RdxSort
    {
        bench_generic(b, vmin, vmax, |data| {
            let mut data = data;
            data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }, n);
    }

    fn bench_rdxsort_generic<T>(b: &mut Bencher, vmin: T, vmax: T, n: usize)
        where T: Clone + PartialOrd + SampleRange,
              Vec<T>: RdxSort
    {
        bench_generic(b, vmin, vmax, |data| {
            let mut data = data;
            data.rdxsort();
        }, n);
    }

    macro_rules! bench_type {
        ($t:ty, $min:expr, $max:expr, [
            $fn_small_rdxsort:ident,
            $fn_small_std:ident,
            $fn_medium_rdxsort:ident,
            $fn_medium_std:ident,
            $fn_large_rdxsort:ident,
            $fn_large_std:ident
        ]) => {
            #[bench]
            fn $fn_small_rdxsort(b: &mut Bencher) {
                bench_rdxsort_generic::<$t>(b, $min, $max, N_SMALL);
            }

            #[bench]
            fn $fn_small_std(b: &mut Bencher) {
                bench_std_generic::<$t>(b, $min, $max, N_SMALL);
            }

            #[bench]
            fn $fn_medium_rdxsort(b: &mut Bencher) {
                bench_rdxsort_generic::<$t>(b, $min, $max, N_MEDIUM);
            }

            #[bench]
            fn $fn_medium_std(b: &mut Bencher) {
                bench_std_generic::<$t>(b, $min, $max, N_MEDIUM);
            }

            #[bench]
            fn $fn_large_rdxsort(b: &mut Bencher) {
                bench_rdxsort_generic::<$t>(b, $min, $max, N_LARGE);
            }

            #[bench]
            fn $fn_large_std(b: &mut Bencher) {
                bench_std_generic::<$t>(b, $min, $max, N_LARGE);
            }
        };
    }

    bench_type!(f32, 0f32, 1f32, [
        bench_small_f32_rdxsort,
        bench_small_f32_std,
        bench_medium_f32_rdxsort,
        bench_medium_f32_std,
        bench_large_f32_rdxsort,
        bench_large_f32_std
    ]);

    bench_type!(f64, 0f64, 1f64, [
        bench_small_f64_rdxsort,
        bench_small_f64_std,
        bench_medium_f64_rdxsort,
        bench_medium_f64_std,
        bench_large_f64_rdxsort,
        bench_large_f64_std
    ]);

    bench_type!(i8, i8::min_value(), i8::max_value(), [
        bench_small_i8_rdxsort,
        bench_small_i8_std,
        bench_medium_i8_rdxsort,
        bench_medium_i8_std,
        bench_large_i8_rdxsort,
        bench_large_i8_std
    ]);

    bench_type!(i16, i16::min_value(), i16::max_value(), [
        bench_small_i16_rdxsort,
        bench_small_i16_std,
        bench_medium_i16_rdxsort,
        bench_medium_i16_std,
        bench_large_i16_rdxsort,
        bench_large_i16_std
    ]);

    bench_type!(i32, i32::min_value(), i32::max_value(), [
        bench_small_i32_rdxsort,
        bench_small_i32_std,
        bench_medium_i32_rdxsort,
        bench_medium_i32_std,
        bench_large_i32_rdxsort,
        bench_large_i32_std
    ]);

    bench_type!(i64, i64::min_value(), i64::max_value(), [
        bench_small_i64_rdxsort,
        bench_small_i64_std,
        bench_medium_i64_rdxsort,
        bench_medium_i64_std,
        bench_large_i64_rdxsort,
        bench_large_i64_std
    ]);

    bench_type!(u8, u8::min_value(), u8::max_value(), [
        bench_small_u8_rdxsort,
        bench_small_u8_std,
        bench_medium_u8_rdxsort,
        bench_medium_u8_std,
        bench_large_u8_rdxsort,
        bench_large_u8_std
    ]);

    bench_type!(u16, u16::min_value(), u16::max_value(), [
        bench_small_u16_rdxsort,
        bench_small_u16_std,
        bench_medium_u16_rdxsort,
        bench_medium_u16_std,
        bench_large_u16_rdxsort,
        bench_large_u16_std
    ]);

    bench_type!(u32, u32::min_value(), u32::max_value(), [
        bench_small_u32_rdxsort,
        bench_small_u32_std,
        bench_medium_u32_rdxsort,
        bench_medium_u32_std,
        bench_large_u32_rdxsort,
        bench_large_u32_std
    ]);

    bench_type!(u64, u64::min_value(), u64::max_value(), [
        bench_small_u64_rdxsort,
        bench_small_u64_std,
        bench_medium_u64_rdxsort,
        bench_medium_u64_std,
        bench_large_u64_rdxsort,
        bench_large_u64_std
    ]);
}
