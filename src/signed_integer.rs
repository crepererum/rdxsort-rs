use template::RdxSortTemplate;

use std::cmp;
use std::mem;

macro_rules! impl_rdxsort {
    ($t:ty, $alias:ty, $min:expr, $zero:expr) => {
        impl RdxSortTemplate for $t {
            #[inline]
            fn cfg_nbuckets() -> usize {
                cmp::max(<$alias as RdxSortTemplate>::cfg_nbuckets(), 3)
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
                    if *self == $min {
                        0
                    } else if *self >= $zero {
                        2
                    } else {
                        1
                    }

                }
            }

            #[inline]
            fn reverse(_round: usize, _bucket: usize) -> bool {
                false
            }
        }
    }
}

impl_rdxsort!(i8, u8, i8::min_value(), 0i8);
impl_rdxsort!(i16, u16, i16::min_value(), 0i16);
impl_rdxsort!(i32, u32, i32::min_value(), 0i32);
impl_rdxsort!(i64, u64, i64::min_value(), 0i64);

#[cfg(target_pointer_width = "16")]
impl RdxSortTemplate for isize {
    #[inline]
    fn cfg_nbuckets() -> usize {
        <i16 as RdxSortTemplate>::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        <i16 as RdxSortTemplate>::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        (*self as i16).get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        <i16 as RdxSortTemplate>::reverse(round, bucket)
    }
}

#[cfg(target_pointer_width = "32")]
impl RdxSortTemplate for isize {
    #[inline]
    fn cfg_nbuckets() -> usize {
        <i32 as RdxSortTemplate>::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        <i32 as RdxSortTemplate>::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        (*self as i32).get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        <i32 as RdxSortTemplate>::reverse(round, bucket)
    }
}

#[cfg(target_pointer_width = "64")]
impl RdxSortTemplate for isize {
    #[inline]
    fn cfg_nbuckets() -> usize {
        <i64 as RdxSortTemplate>::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        <i64 as RdxSortTemplate>::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        (*self as i64).get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        <i64 as RdxSortTemplate>::reverse(round, bucket)
    }
}
