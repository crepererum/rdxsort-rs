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

#[cfg(target_pointer_width = "16")]
impl RdxSortTemplate for usize {
    #[inline]
    fn cfg_nbuckets() -> usize {
        <u16 as RdxSortTemplate>::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        <u16 as RdxSortTemplate>::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        (*self as u16).get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        <u16 as RdxSortTemplate>::reverse(round, bucket)
    }
}

#[cfg(target_pointer_width = "32")]
impl RdxSortTemplate for usize {
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

#[cfg(target_pointer_width = "64")]
impl RdxSortTemplate for usize {
    #[inline]
    fn cfg_nbuckets() -> usize {
        <u64 as RdxSortTemplate>::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        <u64 as RdxSortTemplate>::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        (*self as u64).get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        <u64 as RdxSortTemplate>::reverse(round, bucket)
    }
}
