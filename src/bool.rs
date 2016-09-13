use super::Rdx;

impl Rdx for bool {
    #[inline]
    fn cfg_nbuckets() -> usize {
        2
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        1
    }

    #[inline]
    fn get_bucket(&self, _round: usize) -> usize {
        if *self {
            1
        } else {
            0
        }
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        false
    }
}
