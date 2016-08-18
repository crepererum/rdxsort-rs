use template::RdxSortTemplate;

impl RdxSortTemplate for u8 {
    #[inline]
    fn cfg_nbuckets() -> usize {
        16
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        2
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u8) as usize
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        false
    }
}

impl RdxSortTemplate for u16 {
    #[inline]
    fn cfg_nbuckets() -> usize {
        16
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        4
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u16) as usize
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        false
    }
}

impl RdxSortTemplate for u32 {
    #[inline]
    fn cfg_nbuckets() -> usize {
        16
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        8
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u32) as usize
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        false
    }
}

impl RdxSortTemplate for u64 {
    #[inline]
    fn cfg_nbuckets() -> usize {
        16
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        16
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        let shift = round << 2;
        ((self >> shift) & 15u64) as usize
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        false
    }
}
