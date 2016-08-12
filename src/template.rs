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
}

impl<T> RdxSort for [T] where T: RdxSortTemplate + Clone
{
    fn rdxsort(&mut self) {
        // config
        let cfg_nbuckets = T::cfg_nbuckets();
        let cfg_nrounds = T::cfg_nrounds();

        let n = self.len();
        let presize = cmp::max(16, (n << 2) / cfg_nbuckets);  // TODO: justify the presize value
        let mut buckets_a: Vec<Vec<T>> = Vec::with_capacity(cfg_nbuckets);
        let mut buckets_b: Vec<Vec<T>> = Vec::with_capacity(cfg_nbuckets);
        for _ in 0..cfg_nbuckets {
            buckets_a.push(Vec::with_capacity(presize));
            buckets_b.push(Vec::with_capacity(presize));
        }

        for x in self.iter().cloned() {
            let b = x.get_bucket(0);
            unsafe {
                buckets_a.get_unchecked_mut(b).push(x);
            }
        }

        for round in 1..cfg_nrounds {
            for bucket in &mut buckets_b {
                bucket.clear();
            }
            for bucket in &buckets_a {
                for x in bucket.iter().cloned() {
                    let b = x.get_bucket(round);
                    unsafe {
                        buckets_b.get_unchecked_mut(b).push(x);
                    }
                }
            }
            mem::swap(&mut buckets_a, &mut buckets_b);
        }

        let mut pos = 0;
        for bucket in &mut buckets_a {
            unsafe {
                ptr::copy_nonoverlapping(bucket.as_ptr(),
                                         self.get_unchecked_mut(pos),
                                         bucket.len());
            }
            pos += bucket.len();
        }
    }
}

impl<T> RdxSort for Vec<T> where [T]: RdxSort
{
    fn rdxsort(&mut self) {
        self.as_mut_slice().rdxsort();
    }
}
