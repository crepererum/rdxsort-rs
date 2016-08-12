use super::RdxSort;

use core::ptr;

use std::mem;
use std::ops;

trait HelperTwosComplementTemplate {
    type Alias;

    fn min() -> Self;
    fn zero() -> Self;
}

trait HelperTwosComplement {
    fn rdxsort_(&mut self);
}

// TODO: directly implement `RdxSort` once https://github.com/rust-lang/rfcs/issues/1053 is solved
impl<T> HelperTwosComplement for [T]
    where T: HelperTwosComplementTemplate + Clone + ops::Neg + PartialEq + PartialOrd,
          T::Alias: Clone,
          <T as ops::Neg>::Output: Into<T>,
          Vec<T::Alias>: RdxSort
{
    fn rdxsort_(&mut self) {
        assert_eq!(mem::size_of::<T>(), mem::size_of::<T::Alias>());

        let n = self.len();
        let mut positive: Vec<T::Alias> = Vec::with_capacity(n);
        let mut negative: Vec<T::Alias> = Vec::with_capacity(n);
        let mut min: Vec<T> = Vec::new();

        for x in self.iter().cloned() {
            if x == T::min() {
                // we cannot invert this value later
                min.push(x);
            } else if x >= T::zero() {
                unsafe {
                    positive.push(mem::transmute_copy::<T, T::Alias>(&x));
                }
            } else {
                unsafe {
                    negative.push(mem::transmute_copy::<T, T::Alias>(&((x).into())));
                }
            }
        }

        positive.rdxsort();
        negative.rdxsort();

        assert!(min.len() <= self.len(), "bug: oversized bucket");
        unsafe {
            ptr::copy_nonoverlapping(min.as_ptr(), self.get_unchecked_mut(0), min.len());
        }
        let mut pos = min.len();
        assert!(pos + negative.len() <= self.len(), "bug: oversized bucket");
        unsafe {
            ptr::copy_nonoverlapping(negative.as_ptr() as *mut T,
                                     self.get_unchecked_mut(pos),
                                     negative.len());
        }
        pos += negative.len();
        assert!(pos + positive.len() <= self.len(),
                "bug: bucket sizes doe not sum up");
        unsafe {
            ptr::copy_nonoverlapping(positive.as_ptr() as *mut T,
                                     self.get_unchecked_mut(pos),
                                     positive.len());
        }
    }
}

macro_rules! impl_rdxsort {
    ($t:ty, $alias:ty, $min:expr, $zero:expr) => {
        impl HelperTwosComplementTemplate for $t {
            type Alias = $alias;

            fn min() -> Self {
                $min
            }

            fn zero() -> Self {
                $zero
            }
        }

        impl RdxSort for [$t] {
            fn rdxsort(&mut self) {
                self.rdxsort_();
            }
        }
    }
}

impl_rdxsort!(i8, u8, i8::min_value(), 0i8);
impl_rdxsort!(i16, u16, i16::min_value(), 0i16);
impl_rdxsort!(i32, u32, i32::min_value(), 0i32);
impl_rdxsort!(i64, u64, i64::min_value(), 0i64);
