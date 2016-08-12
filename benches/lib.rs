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

    fn bench_generic<T, F>(b: &mut Bencher, vmin: T, vmax: T, f: F)
        where T: Clone + PartialOrd + SampleRange,
              F: Fn(Vec<T>)
    {
        // config
        let n = 10_000;

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

    fn bench_std_generic<T>(b: &mut Bencher, vmin: T, vmax: T)
        where T: Clone + PartialOrd + SampleRange,
              Vec<T>: RdxSort
    {
        bench_generic(b, vmin, vmax, |data| {
            let mut data = data;
            data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        });
    }

    fn bench_rdxsort_generic<T>(b: &mut Bencher, vmin: T, vmax: T)
        where T: Clone + PartialOrd + SampleRange,
              Vec<T>: RdxSort
    {
        bench_generic(b, vmin, vmax, |data| {
            let mut data = data;
            data.rdxsort();
        });
    }

    #[bench]
    fn bench_i8_std(b: &mut Bencher) {
        bench_std_generic::<i8>(b, i8::min_value(), i8::max_value());
    }

    #[bench]
    fn bench_i8_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<i8>(b, i8::min_value(), i8::max_value());
    }

    #[bench]
    fn bench_i16_std(b: &mut Bencher) {
        bench_std_generic::<i16>(b, i16::min_value(), i16::max_value());
    }

    #[bench]
    fn bench_i16_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<i16>(b, i16::min_value(), i16::max_value());
    }

    #[bench]
    fn bench_i32_std(b: &mut Bencher) {
        bench_std_generic::<i32>(b, i32::min_value(), i32::max_value());
    }

    #[bench]
    fn bench_i32_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<i32>(b, i32::min_value(), i32::max_value());
    }

    #[bench]
    fn bench_i64_std(b: &mut Bencher) {
        bench_std_generic::<i64>(b, i64::min_value(), i64::max_value());
    }

    #[bench]
    fn bench_i64_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<i64>(b, i64::min_value(), i64::max_value());
    }

    #[bench]
    fn bench_u8_std(b: &mut Bencher) {
        bench_std_generic::<u8>(b, u8::min_value(), u8::max_value());
    }

    #[bench]
    fn bench_u8_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<u8>(b, u8::min_value(), u8::max_value());
    }

    #[bench]
    fn bench_u16_std(b: &mut Bencher) {
        bench_std_generic::<u16>(b, u16::min_value(), u16::max_value());
    }

    #[bench]
    fn bench_u16_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<u16>(b, u16::min_value(), u16::max_value());
    }

    #[bench]
    fn bench_u32_std(b: &mut Bencher) {
        bench_std_generic::<u32>(b, u32::min_value(), u32::max_value());
    }

    #[bench]
    fn bench_u32_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<u32>(b, u32::min_value(), u32::max_value());
    }

    #[bench]
    fn bench_u64_std(b: &mut Bencher) {
        bench_std_generic::<u64>(b, u64::min_value(), u64::max_value());
    }

    #[bench]
    fn bench_u64_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<u64>(b, u64::min_value(), u64::max_value());
    }

    #[bench]
    fn bench_f32_std(b: &mut Bencher) {
        bench_std_generic::<f32>(b, 0f32, 1f32);
    }

    #[bench]
    fn bench_f32_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<f32>(b, 0f32, 1f32);
    }

    #[bench]
    fn bench_f64_std(b: &mut Bencher) {
        bench_std_generic::<f64>(b, 0f64, 1f64);
    }

    #[bench]
    fn bench_f64_rdxsort(b: &mut Bencher) {
        bench_rdxsort_generic::<f64>(b, 0f64, 1f64);
    }
}
