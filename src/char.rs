use template::RdxSortTemplate;

impl RdxSortTemplate for char {
    fn cfg_nbuckets() -> usize {
        <u32 as RdxSortTemplate>::cfg_nbuckets()
    }

    fn cfg_nrounds() -> usize {
        <u32 as RdxSortTemplate>::cfg_nrounds()
    }

    fn get_bucket(&self, round: usize) -> usize {
        (*self as u32).get_bucket(round)
    }
}
