extern crate core;

/// Radix Sort implementation for some type
pub trait RdxSort {
    /// Execute Radix Sort, overwrites (unsorted) content of the type.
    fn rdxsort(&mut self);
}

mod template;
mod unsigned_integer;
mod signed_integer;
mod floats;

pub use template::RdxSortTemplate;
