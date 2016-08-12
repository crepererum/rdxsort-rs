extern crate core;

/// RadixSort implementation for some type
pub trait RdxSort {
    /// Execute RadixSort, overwrites (unsorted) content of the type.
    fn rdxsort(&mut self);
}

mod template;
mod unsigned_integer;
mod signed_integer;
mod floats;

pub use template::RdxSortTemplate;
