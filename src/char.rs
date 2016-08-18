use template::RdxSortTemplate;

// same as u32
impl RdxSortTemplate for char {
    fn cfg_nbuckets() -> usize {
        16
    }

    fn cfg_nrounds() -> usize {
        8
    }

    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        (((*self as u32) >> shift) & 15u32) as usize
    }
}
