#![cfg_attr(feature = "unstable", feature(test))]

extern crate rand;
extern crate rdxsort;

#[cfg(feature = "unstable")]
mod unstable {
    extern crate test;
    use self::test::Bencher;

    use rand::{Rng, XorShiftRng};

    use rdxsort::*;

    use std::collections::BTreeSet;
    use std::collections::HashSet;

    static N_MEDIUM: usize = 10_000;

    fn bench_generic<F>(b: &mut Bencher, f: F) where F: Fn(Vec<u32>) {
        let mut set = HashSet::new();
        let mut rng = XorShiftRng::new_unseeded();
        while set.len() < N_MEDIUM {
            set.insert(rng.gen::<u32>());
        }
        let mut vec: Vec<u32> = set.into_iter().collect();
        rng.shuffle(&mut vec[..]);
        let _ = b.iter(|| {
            let vec = vec.clone();
            f(vec);
        });
    }

    #[bench]
    fn bench_set_rdx(b: &mut Bencher) {
        bench_generic(b, |vec| {
            let mut set = RdxTree::new();
            for x in vec {
                set.insert(x);
            }
        });
    }

    #[bench]
    fn bench_set_std(b: &mut Bencher) {
        bench_generic(b, |vec| {
            let mut set = BTreeSet::new();
            for x in vec {
                set.insert(x);
            }
        });
    }
}
