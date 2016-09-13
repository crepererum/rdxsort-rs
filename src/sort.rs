use super::Rdx;

use core::ptr;

use std::cmp;
use std::mem;

/// Radix Sort implementation for some type
pub trait RdxSort {
    /// Execute Radix Sort, overwrites (unsorted) content of the type.
    fn rdxsort(&mut self);
}

#[inline]
fn helper_bucket<T, I>(buckets_b: &mut Vec<Vec<T>>, iter: I, cfg_nbuckets: usize, round: usize)
    where T: Rdx,
          I: Iterator<Item = T>
{
    for x in iter {
        let b = x.get_bucket(round);
        assert!(b < cfg_nbuckets,
                "Your Rdx implementation returns a bucket >= cfg_nbuckets()!");
        unsafe {
            buckets_b.get_unchecked_mut(b).push(x);
        }
    }
}

impl<T> RdxSort for [T] where T: Rdx + Clone
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
