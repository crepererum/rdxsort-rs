use super::RdxSort;

use core::ptr;

use std::cmp;
use std::mem;

/// Generic Radix Sort implementation
///
/// Works by splitting the work in rounds. During every round, the data is sorted into buckets and
/// is then collected again. Rounds are count from `0` to (exclusive) `cfg_nrounds()`.
///
/// The number of buckets is fixed for all rounds. That does not mean that the template has to use
/// all of them, since unused buckets are just empty and have no effect on the collected results.
/// Same hold for the number of rounds. Over-booking one or both of them wastes resources of
/// course.
///
/// **WARNING: The result returned from `get_bucket()` is not checked. Wrong implementations may
/// crash the program, or destroy the world, or both!!!**
pub trait RdxSortTemplate {
    /// Sets the number of buckets used by the generic implementation.
    fn cfg_nbuckets() -> usize;

    /// Sets the number of rounds scheduled by the generic implementation.
    fn cfg_nrounds() -> usize;

    /// Returns the bucket, depending on the round.
    ///
    /// This should respect the radix, e.g.:
    ///
    /// - if the number of buckets is `2` and the type is an unsigned integer, then the result is
    ///   the bit starting with the least significant one.
    /// - if the number of buckets is `8` and the type is an unsigned integer, then the result is
    ///   the byte starting with the least significant one.
    ///
    /// **Never** return a bucker greater or equal the number of buckets. See warning above!
    fn get_bucket(&self, round: usize) -> usize;

    /// Describes the fact that the content of a bucket should be copied back in reverse order
    /// after a certain round.
    fn reverse(round: usize, bucket: usize) -> bool;
}

/// Implements `t1` as alias of `t2`, e.g. `usize = u64` on platforms that have 64 bit pointers.
#[macro_export]
macro_rules! rdxsort_template_alias {
    ($t1:ty = $t2:ty) => {
        impl RdxSortTemplate for $t1 {
            #[inline]
            fn cfg_nbuckets() -> usize {
                <$t2 as RdxSortTemplate>::cfg_nbuckets()
            }

            #[inline]
            fn cfg_nrounds() -> usize {
                <$t2 as RdxSortTemplate>::cfg_nrounds()
            }

            #[inline]
            fn get_bucket(&self, round: usize) -> usize {
                (*self as $t2).get_bucket(round)
            }


            #[inline]
            fn reverse(round: usize, bucket: usize) -> bool {
                <$t2 as RdxSortTemplate>::reverse(round, bucket)
            }
        }
    }
}

#[inline]
fn helper_bucket<T, I>(buckets_b: &mut Vec<Vec<T>>, iter: I, cfg_nbuckets: usize, round: usize)
    where T: RdxSortTemplate,
          I: Iterator<Item = T>
{
    for x in iter {
        let b = x.get_bucket(round);
        assert!(b < cfg_nbuckets,
                "Your RdxSortTemplate implementation returns a bucket >= cfg_nbuckets()!");
        unsafe {
            buckets_b.get_unchecked_mut(b).push(x);
        }
    }
}

impl<T> RdxSort for [T] where T: RdxSortTemplate + Clone
{
    fn rdxsort(&mut self) {
        // config
        let cfg_nbuckets = T::cfg_nbuckets();
        let cfg_nrounds = T::cfg_nrounds();

        // early return
        if cfg_nrounds == 0 {
            return;
        }

        let n = self.len();
        let presize = cmp::max(16, (n << 2) / cfg_nbuckets);  // TODO: justify the presize value
        let mut buckets_a: Vec<Vec<T>> = Vec::with_capacity(cfg_nbuckets);
        let mut buckets_b: Vec<Vec<T>> = Vec::with_capacity(cfg_nbuckets);
        for _ in 0..cfg_nbuckets {
            buckets_a.push(Vec::with_capacity(presize));
            buckets_b.push(Vec::with_capacity(presize));
        }

        helper_bucket(&mut buckets_a, self.iter().cloned(), cfg_nbuckets, 0);

        for round in 1..cfg_nrounds {
            for bucket in &mut buckets_b {
                bucket.clear();
            }
            for (i, bucket) in buckets_a.iter().enumerate() {
                if T::reverse(round - 1, i) {
                    helper_bucket(&mut buckets_b,
                                  bucket.iter().rev().cloned(),
                                  cfg_nbuckets,
                                  round);
                } else {
                    helper_bucket(&mut buckets_b, bucket.iter().cloned(), cfg_nbuckets, round);
                }
            }
            mem::swap(&mut buckets_a, &mut buckets_b);
        }

        let mut pos = 0;
        for (i, bucket) in buckets_a.iter_mut().enumerate() {
            assert!(pos + bucket.len() <= self.len(),
                    "bug: a buckets got oversized");

            if T::reverse(cfg_nrounds - 1, i) {
                for x in bucket.iter().rev().cloned() {
                    unsafe {
                        *self.get_unchecked_mut(pos) = x;
                    }
                    pos += 1;
                }
            } else {
                unsafe {
                    ptr::copy_nonoverlapping(bucket.as_ptr(),
                                             self.get_unchecked_mut(pos),
                                             bucket.len());
                }
                pos += bucket.len();
            }
        }

        assert!(pos == self.len(), "bug: bucket size does not sum up");
    }
}

impl<T> RdxSort for Vec<T> where [T]: RdxSort
{
    fn rdxsort(&mut self) {
        self.as_mut_slice().rdxsort();
    }
}
