use super::RdxSort;

use core::ptr;

use std::cmp;
use std::mem;

pub trait RdxSortTemplate {
    fn cfg_nbuckets() -> usize;
    fn cfg_nrounds() -> usize;
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
