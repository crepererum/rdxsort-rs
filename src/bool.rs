use template::RdxSortTemplate;

impl RdxSortTemplate for bool {
    fn cfg_nbuckets() -> usize {
        2
    }

    fn cfg_nrounds() -> usize {
        1
    }

    fn get_bucket(&self, _round: usize) -> usize {
        if *self {
            1
        } else {
            0
        }
    }
}
