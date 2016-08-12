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
//! test unstable::bench_f32_rdxsort ... bench:     257,137 ns/iter (+/- 26,951)
//! test unstable::bench_f32_std     ... bench:   1,217,554 ns/iter (+/- 53,262)
//! test unstable::bench_f64_rdxsort ... bench:     359,689 ns/iter (+/- 96,726)
//! test unstable::bench_f64_std     ... bench:   1,200,824 ns/iter (+/- 97,724)
//! test unstable::bench_i16_rdxsort ... bench:     155,127 ns/iter (+/- 55,736)
//! test unstable::bench_i16_std     ... bench:     464,016 ns/iter (+/- 46,957)
//! test unstable::bench_i32_rdxsort ... bench:     257,831 ns/iter (+/- 43,987)
//! test unstable::bench_i32_std     ... bench:     465,150 ns/iter (+/- 41,255)
//! test unstable::bench_i64_rdxsort ... bench:     361,660 ns/iter (+/- 57,155)
//! test unstable::bench_i64_std     ... bench:     481,594 ns/iter (+/- 64,260)
//! test unstable::bench_i8_rdxsort  ... bench:     101,423 ns/iter (+/- 11,662)
//! test unstable::bench_i8_std      ... bench:     633,929 ns/iter (+/- 81,700)
//! test unstable::bench_u16_rdxsort ... bench:     144,205 ns/iter (+/- 24,550)
//! test unstable::bench_u16_std     ... bench:     513,002 ns/iter (+/- 61,807)
//! test unstable::bench_u32_rdxsort ... bench:     285,324 ns/iter (+/- 30,066)
//! test unstable::bench_u32_std     ... bench:     513,305 ns/iter (+/- 47,894)
//! test unstable::bench_u64_rdxsort ... bench:     449,705 ns/iter (+/- 51,422)
//! test unstable::bench_u64_std     ... bench:     531,199 ns/iter (+/- 66,398)
//! test unstable::bench_u8_rdxsort  ... bench:      76,996 ns/iter (+/- 10,708)
//! test unstable::bench_u8_std      ... bench:     671,313 ns/iter (+/- 101,326)
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
