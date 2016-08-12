//! # RdxSort
//!
//! This crate implements [Radix Sort](https://en.wikipedia.org/wiki/Radix_sort) for slices of
//! different data types, either directly or by exploiting the implementation of other data types.
//!
//! The main reason for implementing yet another sorting algorithm is that most sorting algorithms
//! are comparative methods. To sort data, they rely on a function that compares data elements. It
//! can be proven that this leads to a runtime complexity of `O(n*log(n))` in avarage and `O(n^2)`
//! in the worst case. In contrast to that, Radix Sort exploits that fact that many data types have
//! a limited range of possible values and within that range a limited resolution (that also holds
//! for floating point numbers). For a detailed explaination see the
//! [Wikipedia article](https://en.wikipedia.org/wiki/Radix_sort). The result of this special
//! treatment is a lowered and constant complexity of `O(n*k)` where `k` is the number of fixed
//! rounds required to sort a specific data type.
//!
//!
//! ## Supported Data Types
//!
//! Currently, the following data types are supported:
//!
//! - **unsigned integers:** native implementation
//! - **signed integers:** splitting into positive and negative parts and using the unsigned
//!   implementation
//! - **floats:** splits data into `-∞`, `(-∞,-0)`, `-0`, `+0`, `(+0,+∞)`, `+∞` and treating the two
//!   ranges as unsigned integer values. [Subnormals](https://en.wikipedia.org/wiki/Denormal_number)
//!   and `NaN`s are not supported!
//!
//!
//! ## Example
//!
//! ```
//! use rdxsort::*;
//!
//! fn main() {
//!     let mut data = vec![2, 10, 0, 1];
//!     data.rdxsort();
//!     assert!(data == vec![0, 1, 2, 10]);
//! }
//! ```
//!
//!
//! ## Performance
//!
//! Of course the lower runtime complexity of Radix Sort shows its power when sorting certain data
//! types. The advantage depends on the size and complexity of the type. While short unsigned
//! integers benifit the most, long types do not show that huge improvements. The following listing
//! shows the runtime required for sorting 10'000 Elements, once using the standard library sort
//! and once using Radix Sort:
//!
//! ```text
//! test unstable::bench_f32_rdxsort ... bench:     253,863 ns/iter (+/- 178,952)
//! test unstable::bench_f32_std     ... bench:   1,213,105 ns/iter (+/- 383,496)
//! test unstable::bench_f64_rdxsort ... bench:     450,055 ns/iter (+/- 219,251)
//! test unstable::bench_f64_std     ... bench:   1,229,838 ns/iter (+/- 23,368)
//! test unstable::bench_i16_rdxsort ... bench:     157,572 ns/iter (+/- 3,406)
//! test unstable::bench_i16_std     ... bench:     493,214 ns/iter (+/- 8,438)
//! test unstable::bench_i32_rdxsort ... bench:     264,850 ns/iter (+/- 17,677)
//! test unstable::bench_i32_std     ... bench:     494,648 ns/iter (+/- 22,495)
//! test unstable::bench_i64_rdxsort ... bench:     483,610 ns/iter (+/- 109,888)
//! test unstable::bench_i64_std     ... bench:     520,562 ns/iter (+/- 163,783)
//! test unstable::bench_i8_rdxsort  ... bench:     107,368 ns/iter (+/- 22,236)
//! test unstable::bench_i8_std      ... bench:     668,804 ns/iter (+/- 88,602)
//! test unstable::bench_u16_rdxsort ... bench:     149,114 ns/iter (+/- 915)
//! test unstable::bench_u16_std     ... bench:     561,147 ns/iter (+/- 112,822)
//! test unstable::bench_u32_rdxsort ... bench:     306,105 ns/iter (+/- 26,152)
//! test unstable::bench_u32_std     ... bench:     550,718 ns/iter (+/- 54,146)
//! test unstable::bench_u64_rdxsort ... bench:     585,753 ns/iter (+/- 459,075)
//! test unstable::bench_u64_std     ... bench:     557,929 ns/iter (+/- 104,396)
//! test unstable::bench_u8_rdxsort  ... bench:      80,124 ns/iter (+/- 4,273)
//! test unstable::bench_u8_std      ... bench:     712,865 ns/iter (+/- 21,935)
//! ```
//!
//!
//! ## Implementing New Types
//!
//! This crate enables you to add support for new types by implementing `RdxSortTemplate`. It
//! describes how data is sorted into buckets and how many rounds of sorting are scheduled.
//!
//! ```
//! use rdxsort::*;
//!
//! // `Clone` is required for `RdxSort`
//! // `PartialEq` is only required for the equality assert, not for the actual sorting
//! #[derive(Clone, PartialEq)]
//! struct Foo {
//!     a: u8,
//!     b: u8,
//! }
//!
//! impl RdxSortTemplate for Foo {
//!     fn cfg_nbuckets() -> usize {
//!         // usually too high, but works as a simple demonstration
//!         // `256 = 2^8`
//!         256
//!     }
//!
//!     fn cfg_nrounds() -> usize {
//!         // one per sub-type
//!         2
//!     }
//!
//!     fn get_bucket(&self, round: usize) -> usize {
//!         // return the least significant digit first
//!         if round == 0 {
//!             self.b as usize
//!         } else {
//!             self.a as usize
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let mut data = vec![
//!         Foo{a: 5, b: 2},
//!         Foo{a: 0, b: 1},
//!         Foo{a: 5, b: 1},
//!         Foo{a: 0, b: 2},
//!     ];
//!     data.rdxsort();
//!
//!     let reference = vec![
//!         Foo{a: 0, b: 1},
//!         Foo{a: 0, b: 2},
//!         Foo{a: 5, b: 1},
//!         Foo{a: 5, b: 2},
//!     ];
//!     assert!(data == reference);
//! }
//! ```

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
