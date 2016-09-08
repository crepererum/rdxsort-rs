use template::RdxSortTemplate;

use std::cmp;
use std::mem;

macro_rules! impl_rdxsort {
    ($t:ty, $alias:ty, $mask:expr) => {
        impl RdxSortTemplate for $t {
            #[inline]
            fn cfg_nbuckets() -> usize {
                cmp::max(<$alias as RdxSortTemplate>::cfg_nbuckets(), 2)
            }

            #[inline]
            fn cfg_nrounds() -> usize {
                <$alias as RdxSortTemplate>::cfg_nrounds() + 1
            }

            #[inline]
            fn get_bucket(&self, round: usize) -> usize {
                let alias = unsafe { mem::transmute::<$t, $alias>(*self) };
                if round < <$alias as RdxSortTemplate>::cfg_nrounds() {
                    alias.get_bucket(round)
                } else {
                    if self.is_nan() {
                        panic!("Sorting of NaNs is not implemented!");
                    } else {
                        if (alias & $mask) == 0 {
                            1
                        } else {
                            0
                        }
                    }
                }
            }

            #[inline]
            fn reverse(round: usize, bucket: usize) -> bool {
                round == <$alias as RdxSortTemplate>::cfg_nrounds() && bucket == 0
            }
        }
    }
}

impl_rdxsort!(f32, u32, 0x80000000u32);
impl_rdxsort!(f64, u64, 0x8000000000000000u64);
