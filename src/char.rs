use template::RdxSortTemplate;

impl RdxSortTemplate for char {
    #[inline]
    fn cfg_nbuckets() -> usize {
        <u32 as RdxSortTemplate>::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        <u32 as RdxSortTemplate>::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        (*self as u32).get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        <u32 as RdxSortTemplate>::reverse(round, bucket)
    }
}
