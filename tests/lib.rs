extern crate rand;
extern crate rdx_sort;

use rand::{Rng, XorShiftRng};
use rand::distributions::{IndependentSample, Range};
use rand::distributions::range::SampleRange;

use rdx_sort::*;

fn generate_data<T>(n: usize, r: &Range<T>, rng: &mut XorShiftRng) -> Vec<T>
    where T: SampleRange
{
    let mut data = Vec::with_capacity(n);

    for _ in (0 as usize)..n {
        data.push(r.ind_sample(rng));
    }

    return data;
}

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

fn test_generic<T>(vmin: T, vmax: T, vspecial: Vec<T>)
    where T: Clone,
          T: PartialOrd,
          T: SampleRange,
          T: Send,
          T: 'static,
          Vec<T>: RdxSort
{
    // config
    let n = 100_000;

    // generate data
    let r = Range::new(vmin.clone(), vmax.clone());
    let mut rng = XorShiftRng::new_unseeded();
    let mut data = generate_data(n, &r, &mut rng);
    let mut positions = Vec::with_capacity(n);
    for i in 0..n {
        positions.push(i);
    }
    rng.shuffle(&mut positions[..]);
    assert!(vspecial.len() + 2 < n, "to many special values to test!");
    data[positions[0]] = vmin;
    data[positions[1]] = vmax;
    for (i, x) in positions.into_iter().skip(2).zip(vspecial.into_iter()) {
        data[i] = x;
    }
    assert!(data.len() == n, "generated data has wrong length!");

    let mut data_sorted_ref = data.clone();
    data_sorted_ref.sort_by(|a, b| a.partial_cmp(b).unwrap());

    data.rdxsort();
    assert!(data.len() == n, "sorted data has wrong lenght!");
    assert!(is_sorted(&data), "data is not sorted!");
    for (x, y) in data.iter().zip(data_sorted_ref.iter()) {
        assert!(x == y, "sortd data does not match the reference!");
    }
}

#[test]
fn test_i8() {
    test_generic::<i8>(i8::min_value(), i8::max_value(), vec![i8::min_value() + 1, -1i8, 0i8, 1i8, i8::max_value() - 1]);
}

#[test]
fn test_i16() {
    test_generic::<i16>(i16::min_value(), i16::max_value(), vec![i16::min_value() + 1, -1i16, 0i16, 1i16, i16::max_value() - 1]);
}

#[test]
fn test_i32() {
    test_generic::<i32>(i32::min_value(), i32::max_value(), vec![i32::min_value() + 1, -1i32, 0i32, 1i32, i32::max_value() - 1]);
}

#[test]
fn test_i64() {
    test_generic::<i64>(i64::min_value(), i64::max_value(), vec![i64::min_value() + 1, -1i64, 0i64, 1i64, i64::max_value() - 1]);
}

#[test]
fn test_u8() {
    test_generic::<u8>(u8::min_value(), u8::max_value(), vec![0u8, 1u8, u8::max_value() - 1]);
}

#[test]
fn test_u16() {
    test_generic::<u16>(u16::min_value(), u16::max_value(), vec![0u16, 1u16, u16::max_value() - 1]);
}

#[test]
fn test_u32() {
    test_generic::<u32>(u32::min_value(), u32::max_value(), vec![0u32, 1u32, u32::max_value() - 1]);
}

#[test]
fn test_u64() {
    test_generic::<u64>(u64::min_value(), u64::max_value(), vec![0u64, 1u64, u64::max_value() - 1]);
}
