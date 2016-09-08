extern crate rand;
extern crate rdxsort;

use std::collections;
use std::hash;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter;
use std::mem;
use std::ops;

use rand::{Rand, Rng, XorShiftRng};

use rdxsort::*;

pub const CFG_N: usize = 10_000;
pub const CFG_M: usize = 10;

fn is_sorted<T>(data: &Vec<T>) -> bool
    where T: Clone,
          T: PartialOrd
{
    let mut last_entry: Option<T> = None;
    for x in data.iter().cloned() {
        match last_entry {
            Some(l) => {
                if !(l.le(&x)) {
                    return false;
                }
            }
            None => {}
        }
        last_entry = Some(x);
    }
    return true;
}

pub trait MyHash {
    fn hash_it<H>(&self, state: &mut H) where H: Hasher;
}

macro_rules! trivial_myhash {
    ($t:ty) => {
        impl MyHash for $t {
            fn hash_it<H>(&self, state: &mut H) where H: Hasher {
                self.hash(state);
            }
        }
    }
}

trivial_myhash!([u8; 0]);
trivial_myhash!([u8; 4]);
trivial_myhash!((u8,));
trivial_myhash!((u8, i32));
trivial_myhash!((u8, i32, char));
trivial_myhash!(bool);
trivial_myhash!(char);
trivial_myhash!(i8);
trivial_myhash!(i16);
trivial_myhash!(i32);
trivial_myhash!(i64);
trivial_myhash!(isize);
trivial_myhash!(u8);
trivial_myhash!(u16);
trivial_myhash!(u32);
trivial_myhash!(u64);
trivial_myhash!(usize);

impl MyHash for f32 {
    fn hash_it<H>(&self, state: &mut H) where H: Hasher {
        let alias = unsafe {mem::transmute_copy::<f32, u32>(self)};
        alias.hash(state);
    }
}

impl MyHash for f64 {
    fn hash_it<H>(&self, state: &mut H) where H: Hasher {
        let alias = unsafe {mem::transmute_copy::<f64, u64>(self)};
        alias.hash(state);
    }
}

fn guess_entropy<T>(data: &Vec<T>) -> f64 where T: MyHash {
    let mut counter = collections::HashMap::<u64, usize>::new();

    for x in data {
        let mut hasher = hash::SipHasher::new();
        x.hash_it(&mut hasher);
        let key = hasher.finish();

        let state = counter.entry(key).or_insert(0);
        *state += 1;
    }

    counter.values().map(|&x| x as f64 / data.len() as f64).map(|x| -x * x.log2()).fold(0f64, |a, b| a + b)
}

pub fn test_generic<T>(data: Vec<T>)
    where T: Clone + PartialOrd,
          Vec<T>: RdxSort
{
    let mut data = data;
    let n = data.len();
    let mut data_sorted_ref = data.clone();
    data_sorted_ref.sort_by(|a, b| a.partial_cmp(b).unwrap());

    data.rdxsort();
    assert!(data.len() == n, "sorted data has wrong lenght!");
    assert!(is_sorted(&data), "data is not sorted!");
    for (x, y) in data.iter().zip(data_sorted_ref.iter()) {
        assert!(x == y, "sortd data does not match the reference!");
    }
}

pub fn test_empty_generic<T>()
    where T: Clone + PartialOrd,
          Vec<T>: RdxSort
{
    let data: Vec<T> = vec![];
    test_generic(data);
}

pub fn test_rnd_generic<T>(vspecial: Vec<T>)
    where T: Clone + PartialOrd + Rand + MyHash,
          Vec<T>: RdxSort
{
    // config
    let entropy_threshold = 0.5f64;

    // generate data
    let mut rng = XorShiftRng::new_unseeded();
    let mut data: Vec<T> = rng.gen_iter::<T>().take(CFG_N).collect();
    let mut positions: Vec<usize> = (0..(CFG_N + 1)).collect();
    rng.shuffle(&mut positions[..]);
    assert!(vspecial.len() * CFG_M < CFG_N, "to many special values to test!");
    for (i, x) in vspecial.into_iter().enumerate() {
        for j in 0..CFG_M {
            let pos = positions[i * CFG_M + j];
            data[pos] = x.clone();
        }
    }
    assert!(data.len() == CFG_N, "generated data has wrong length!");
    assert!(guess_entropy(&data) >= entropy_threshold, "generated data does not contain enough entropy!");

    test_generic(data);
}

pub fn test_single_generic<T>(x: T)
    where T: Clone + PartialOrd,
          Vec<T>: RdxSort
{
    let data: Vec<T> = vec![x];
    test_generic(data);
}

pub fn test_full_generic<T>(vmin: T, vmax: T)
    where T: Clone+ PartialOrd + ops::Add,
          ops::Range<T>: iter::Iterator,
          Vec<T>: iter::FromIterator<<ops::Range<T> as iter::Iterator>::Item>,
          Vec<T>: RdxSort
{
    // TODO: use inclusive range once it's stable
    let mut data: Vec<T> = (vmin.clone()..vmax.clone()).collect();
    data.push(vmax);

    let mut rng = XorShiftRng::new_unseeded();
    rng.shuffle(&mut data[..]);

    test_generic(data);
}

mod sub_array {
    use super::*;

    #[test]
    fn test_empty_array0() {
        test_empty_generic::<[u8; 0]>();
    }

    #[test]
    fn test_single_array0() {
        test_single_generic::<[u8; 0]>([]);
    }

    #[test]
    fn test_rnd_array4() {
        test_rnd_generic::<[u8; 4]>(vec![]);
    }

    #[test]
    fn test_empty_array4() {
        test_empty_generic::<[u8; 4]>();
    }

    #[test]
    fn test_single_array4() {
        test_single_generic::<[u8; 4]>([1, 2, 3, 4]);
    }
}

mod sub_bool {
    use super::*;

    #[test]
    fn test_rnd_bool() {
        test_rnd_generic::<bool>(vec![false, true]);
    }

    #[test]
    fn test_empty_bool() {
        test_empty_generic::<bool>();
    }

    #[test]
    fn test_single_bool() {
        test_single_generic::<bool>(true);
    }
}

mod sub_char {
    use super::*;

    use std::char;

    #[test]
    fn test_rnd_char() {
        test_rnd_generic::<char>(vec!['\0', char::MAX]);
    }

    #[test]
    fn test_empty_char() {
        test_empty_generic::<char>();
    }

    #[test]
    fn test_single_char() {
        test_single_generic::<char>('x');
    }
}

mod sub_i8 {
    use super::*;

    #[test]
    fn test_rnd_i8() {
        test_rnd_generic::<i8>(vec![i8::min_value(), i8::min_value() + 1, -1i8, 0i8, 1i8, i8::max_value() - 1, i8::max_value()]);
    }

    #[test]
    fn test_full_i8() {
        test_full_generic::<i8>(i8::min_value(), i8::max_value());
    }

    #[test]
    fn test_empty_i8() {
        test_empty_generic::<i8>();
    }

    #[test]
    fn test_single_i8() {
        test_single_generic::<i8>(3i8);
    }
}

mod sub_i16 {
    use super::*;

    #[test]
    fn test_rnd_i16() {
        test_rnd_generic::<i16>(vec![i16::min_value(), i16::min_value() + 1, -1i16, 0i16, 1i16, i16::max_value() - 1, i16::max_value()]);
    }

    #[test]
    fn test_full_i16() {
        test_full_generic::<i16>(i16::min_value(), i16::max_value());
    }

    #[test]
    fn test_empty_i16() {
        test_empty_generic::<i16>();
    }

    #[test]
    fn test_single_i16() {
        test_single_generic::<i16>(3i16);
    }
}

mod sub_i32 {
    use super::*;

    #[test]
    fn test_rnd_i32() {
        test_rnd_generic::<i32>(vec![i32::min_value(), i32::min_value() + 1, -1i32, 0i32, 1i32, i32::max_value() - 1, i32::max_value()]);
    }

    #[test]
    fn test_empty_i32() {
        test_empty_generic::<i32>();
    }

    #[test]
    fn test_single_i32() {
        test_single_generic::<i32>(3i32);
    }
}

mod sub_i64 {
    use super::*;

    #[test]
    fn test_rnd_i64() {
        test_rnd_generic::<i64>(vec![i64::min_value(), i64::min_value() + 1, -1i64, 0i64, 1i64, i64::max_value() - 1, i64::max_value()]);
    }

    #[test]
    fn test_empty_i64() {
        test_empty_generic::<i64>();
    }

    #[test]
    fn test_single_i64() {
        test_single_generic::<i64>(3i64);
    }
}

mod sub_isize {
    use super::*;

    #[test]
    fn test_rnd_isize() {
        test_rnd_generic::<isize>(vec![isize::min_value(), isize::min_value() + 1, -1, 0, 1, isize::max_value() - 1, isize::max_value()]);
    }

    #[test]
    fn test_empty_isize() {
        test_empty_generic::<isize>();
    }

    #[test]
    fn test_single_isize() {
        test_single_generic::<isize>(3);
    }
}

mod sub_u8 {
    use super::*;

    #[test]
    fn test_rnd_u8() {
        test_rnd_generic::<u8>(vec![0u8, 1u8, u8::max_value() - 1, u8::max_value()]);
    }

    #[test]
    fn test_full_u8() {
        test_full_generic::<u8>(u8::min_value(), u8::max_value());
    }

    #[test]
    fn test_empty_u8() {
        test_empty_generic::<u8>();
    }

    #[test]
    fn test_single_u8() {
        test_single_generic::<u8>(3u8);
    }
}

mod sub_u16 {
    use super::*;

    #[test]
    fn test_rnd_u16() {
        test_rnd_generic::<u16>(vec![0u16, 1u16, u16::max_value() - 1, u16::max_value()]);
    }

    #[test]
    fn test_full_u16() {
        test_full_generic::<u16>(u16::min_value(), u16::max_value());
    }

    #[test]
    fn test_empty_u16() {
        test_empty_generic::<u16>();
    }

    #[test]
    fn test_single_u16() {
        test_single_generic::<u16>(3u16);
    }
}

mod sub_u32 {
    use super::*;

    #[test]
    fn test_rnd_u32() {
        test_rnd_generic::<u32>(vec![0u32, 1u32, u32::max_value() - 1, u32::max_value()]);
    }

    #[test]
    fn test_empty_u32() {
        test_empty_generic::<u32>();
    }

    #[test]
    fn test_single_u32() {
        test_single_generic::<u32>(3u32);
    }
}

mod sub_u64 {
    use super::*;

    #[test]
    fn test_rnd_u64() {
        test_rnd_generic::<u64>(vec![0u64, 1u64, u64::max_value() - 1, u64::max_value()]);
    }

    #[test]
    fn test_empty_u64() {
        test_empty_generic::<u64>();
    }

    #[test]
    fn test_single_u64() {
        test_single_generic::<u64>(3u64);
    }
}

mod sub_usize {
    use super::*;

    #[test]
    fn test_rnd_usize() {
        test_rnd_generic::<usize>(vec![0, 1, usize::max_value() - 1, usize::max_value()]);
    }

    #[test]
    fn test_empty_usize() {
        test_empty_generic::<usize>();
    }

    #[test]
    fn test_single_usize() {
        test_single_generic::<usize>(3);
    }
}

mod sub_f32 {
    use super::*;

    use std::f32;

    #[test]
    fn test_rnd_f32() {
        test_rnd_generic::<f32>(vec![-f32::INFINITY, -0f32, 0f32, f32::INFINITY]);
    }

    #[test]
    fn test_empty_f32() {
        test_empty_generic::<f32>();
    }

    #[test]
    fn test_single_f32() {
        test_single_generic::<f32>(3f32);
    }
}

mod sub_f64 {
    use super::*;

    use std::f64;

    #[test]
    fn test_rnd_f64() {
        test_rnd_generic::<f64>(vec![-f64::INFINITY, -0f64, 0f64, f64::INFINITY]);
    }

    #[test]
    fn test_empty_f64() {
        test_empty_generic::<f64>();
    }

    #[test]
    fn test_single_f64() {
        test_single_generic::<f64>(3f64);
    }
}

mod sub_tuple {
    use super::*;

    #[test]
    fn test_empty_tuple0() {
        test_empty_generic::<()>();
    }

    #[test]
    fn test_single_tuple0() {
        test_single_generic::<()>(());
    }

    #[test]
    fn test_rnd_tuple1() {
        test_rnd_generic::<(u8,)>(vec![]);
    }

    #[test]
    fn test_empty_tuple1() {
        test_empty_generic::<(u8,)>();
    }

    #[test]
    fn test_single_tuple1() {
        test_single_generic::<(u8,)>((1u8,));
    }

    #[test]
    fn test_rnd_tuple2() {
        test_rnd_generic::<(u8, i32)>(vec![]);
    }

    #[test]
    fn test_empty_tuple2() {
        test_empty_generic::<(u8, i32)>();
    }

    #[test]
    fn test_single_tuple2() {
        test_single_generic::<(u8, i32)>((1u8, 1337i32));
    }

    #[test]
    fn test_rnd_tuple3() {
        test_rnd_generic::<(u8, i32, char)>(vec![]);
    }

    #[test]
    fn test_empty_tuple3() {
        test_empty_generic::<(u8, i32, char)>();
    }

    #[test]
    fn test_single_tuple3() {
        test_single_generic::<(u8, i32, char)>((1u8, 1337i32, 'x'));
    }
}
