use super::Rdx;

use std::cmp;

impl Rdx for () {
    #[inline]
    fn cfg_nbuckets() -> usize {
        0
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        0
    }

    #[inline]
    fn get_bucket(&self, _round: usize) -> usize {
        unreachable!()
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        unreachable!()
    }
}

impl<A> Rdx for (A,)
    where A: Rdx
{
    #[inline]
    fn cfg_nbuckets() -> usize {
        A::cfg_nbuckets()
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        A::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        self.0.get_bucket(round)
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        A::reverse(round, bucket)
    }
}

impl<A, B> Rdx for (A, B)
    where A: Rdx,
          B: Rdx
{
    #[inline]
    fn cfg_nbuckets() -> usize {
        cmp::max(A::cfg_nbuckets(), B::cfg_nbuckets())
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        A::cfg_nrounds() + B::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        if round < B::cfg_nrounds() {
            self.1.get_bucket(round)
        } else {
            self.0.get_bucket(round - B::cfg_nrounds())
        }
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        if round < B::cfg_nrounds() {
            B::reverse(round, bucket)
        } else {
            A::reverse(round - B::cfg_nrounds(), bucket)
        }
    }
}

impl<A, B, C> Rdx for (A, B, C)
    where A: Rdx,
          B: Rdx,
          C: Rdx
{
    #[inline]
    fn cfg_nbuckets() -> usize {
        cmp::max(A::cfg_nbuckets(),
                 cmp::max(B::cfg_nbuckets(), C::cfg_nbuckets()))
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        A::cfg_nrounds() + B::cfg_nrounds() + C::cfg_nrounds()
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        if round < C::cfg_nrounds() {
            self.2.get_bucket(round)
        } else if round < B::cfg_nrounds() + C::cfg_nrounds() {
            self.1.get_bucket(round - C::cfg_nrounds())
        } else {
            self.0.get_bucket(round - B::cfg_nrounds() - C::cfg_nrounds())
        }
    }

    #[inline]
    fn reverse(round: usize, bucket: usize) -> bool {
        if round < C::cfg_nrounds() {
            C::reverse(round, bucket)
        } else if round < B::cfg_nrounds() + C::cfg_nrounds() {
            B::reverse(round - C::cfg_nrounds(), bucket)
        } else {
            A::reverse(round - B::cfg_nrounds() - C::cfg_nrounds(), bucket)
        }
    }
}
