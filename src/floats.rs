use super::RdxSort;

use std::cmp;
use std::mem;
use std::ops;

trait FloatHack {
    type Alias;

    fn zero() -> Self;
    fn query_normal(&self) -> bool;
    fn query_infinite(&self) -> bool;
}

trait HelperFloatHack {
    fn rdxsort_(&mut self);
}

// TODO: directly implement `RdxSort` once https://github.com/rust-lang/rfcs/issues/1053 is solved
impl<T> HelperFloatHack for Vec<T>
    where T: FloatHack + Clone + cmp::PartialEq + cmp::PartialOrd + ops::Neg,
          T::Alias: Clone,
          <T as ops::Neg>::Output: Into<T>,
          Vec<T::Alias>: RdxSort
{
    fn rdxsort_(&mut self) {
        assert_eq!(mem::size_of::<T>(), mem::size_of::<T::Alias>());

        let n = self.len();
        let mut bucket_inf_negative: Vec<T> = Vec::new();
        let mut bucket_negative: Vec<T::Alias> = Vec::with_capacity(n);
        let mut bucket_zero_negative: Vec<T> = Vec::new();
        let mut bucket_zero_positive: Vec<T> = Vec::new();
        let mut bucket_positive: Vec<T::Alias> = Vec::with_capacity(n);
        let mut bucket_inf_positive: Vec<T> = Vec::new();
        for x in self.iter().cloned() {
            if x.query_normal() {
                if x > T::zero() {
                    bucket_positive.push(unsafe { mem::transmute_copy::<T, T::Alias>(&x) });
                } else {
                    bucket_negative.push(unsafe { mem::transmute_copy::<T, T::Alias>(&x) });
                }
            } else if x == T::zero() {
                bucket_zero_positive.push(x);
            } else if x == (-T::zero()).into() {
                bucket_zero_negative.push(x);
            } else if x.query_infinite() {
                if x > T::zero() {
                    bucket_inf_positive.push(x);
                } else {
                    bucket_inf_negative.push(x);
                }
            } else {
                panic!("Sorting of NaNs and subnormals is not implemented!");
            }
        }

        bucket_negative.rdxsort();
        bucket_positive.rdxsort();

        self.clear();
        self.append(&mut bucket_inf_negative);
        self.extend(bucket_negative.iter()
                                   .rev()
                                   .cloned()
                                   .map(|x| unsafe { mem::transmute_copy::<T::Alias, T>(&x) }));
        self.append(&mut bucket_zero_negative);
        self.append(&mut bucket_zero_positive);
        self.extend(bucket_positive.iter()
                                   .cloned()
                                   .map(|x| unsafe { mem::transmute_copy::<T::Alias, T>(&x) }));
        self.append(&mut bucket_inf_positive);
    }
}

impl FloatHack for f64 {
    type Alias = u64;

    fn zero() -> Self {
        0f64
    }

    fn query_normal(&self) -> bool {
        self.is_normal()
    }

    fn query_infinite(&self) -> bool {
        self.is_infinite()
    }
}

impl RdxSort for Vec<f64> {
    fn rdxsort(&mut self) {
        self.rdxsort_();
    }
}
