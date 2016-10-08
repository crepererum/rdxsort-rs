use super::Rdx;

macro_rules! impl_rdxsort {
    ($n:expr) => {
        impl<T> Rdx for [T; $n] where T: Rdx {
            #[inline]
            fn cfg_nbuckets() -> usize {
                T::cfg_nbuckets()
            }

            #[inline]
            fn cfg_nrounds() -> usize {
                T::cfg_nrounds() * $n
            }

            #[inline]
            fn get_bucket(&self, round: usize) -> usize {
                let i = round / T::cfg_nrounds();
                let j = round % T::cfg_nrounds();
                self[$n - i - 1].get_bucket(j)
            }

            #[inline]
            fn reverse(round: usize, bucket: usize) -> bool {
                let j = round % T::cfg_nrounds();
                T::reverse(j, bucket)
            }
        }
    }
}

impl_rdxsort!(0);
impl_rdxsort!(1);
impl_rdxsort!(2);
impl_rdxsort!(3);
impl_rdxsort!(4);
impl_rdxsort!(5);
impl_rdxsort!(6);
impl_rdxsort!(7);
impl_rdxsort!(8);
impl_rdxsort!(9);
impl_rdxsort!(10);
impl_rdxsort!(11);
impl_rdxsort!(12);
impl_rdxsort!(13);
impl_rdxsort!(14);
impl_rdxsort!(15);
impl_rdxsort!(16);
impl_rdxsort!(17);
impl_rdxsort!(18);
impl_rdxsort!(19);
impl_rdxsort!(20);
impl_rdxsort!(21);
impl_rdxsort!(22);
impl_rdxsort!(23);
impl_rdxsort!(24);
impl_rdxsort!(25);
impl_rdxsort!(26);
impl_rdxsort!(27);
impl_rdxsort!(28);
impl_rdxsort!(29);
impl_rdxsort!(30);
impl_rdxsort!(31);
impl_rdxsort!(32);
