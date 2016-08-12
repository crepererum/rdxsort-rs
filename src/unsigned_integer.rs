use template::RdxSortTemplate;

impl RdxSortTemplate for u8 {
    fn cfg_nbuckets() -> usize {
        16
    }

    fn cfg_nrounds() -> usize {
        2
    }

    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u8) as usize
    }
}

impl RdxSortTemplate for u16 {
    fn cfg_nbuckets() -> usize {
        16
    }

    fn cfg_nrounds() -> usize {
        4
    }

    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u16) as usize
    }
}

impl RdxSortTemplate for u32 {
    fn cfg_nbuckets() -> usize {
        16
    }

    fn cfg_nrounds() -> usize {
        8
    }

    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u32) as usize
    }
}

impl RdxSortTemplate for u64 {
    fn cfg_nbuckets() -> usize {
        64
    }

    fn cfg_nrounds() -> usize {
        11
    }

    fn get_bucket(&self, round: usize) -> usize {
        let shift = round * 6;
        ((self >> shift) & 31u64) as usize
    }
}
