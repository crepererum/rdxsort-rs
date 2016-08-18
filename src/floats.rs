use template::RdxSortTemplate;

use std::cmp;
use std::mem;

macro_rules! impl_rdxsort {
    ($t:ty, $alias:ty, $zero:expr) => {
        impl RdxSortTemplate for $t {
            #[inline]
            fn cfg_nbuckets() -> usize {
                cmp::max(<$alias as RdxSortTemplate>::cfg_nbuckets(), 6)
            }

            #[inline]
            fn cfg_nrounds() -> usize {
                <$alias as RdxSortTemplate>::cfg_nrounds() + 1
            }

            #[inline]
            fn get_bucket(&self, round: usize) -> usize {
                if round < <$alias as RdxSortTemplate>::cfg_nrounds() {
                    let alias = unsafe { mem::transmute::<$t, $alias>(*self) };
                    alias.get_bucket(round)
                } else {
                    if self.is_normal() {
                        if *self > $zero {
                            4
                        } else {
                            1
                        }
                    } else if *self == $zero {
                        3
                    } else if *self == -$zero {
                        2
                    } else if self.is_infinite() {
                        if *self > $zero {
                            5
                        } else {
                            0
                        }
                    } else {
                        panic!("Sorting of NaNs and subnormals is not implemented!");
                    }
                }
            }

            #[inline]
            fn reverse(round: usize, bucket: usize) -> bool {
                round == <$alias as RdxSortTemplate>::cfg_nrounds() && bucket == 1
            }
        }
    }
}

impl_rdxsort!(f32, u32, 0f32);
impl_rdxsort!(f64, u64, 0f64);
