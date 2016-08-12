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
//! shows the runtime required for sorting data sets of different sizes, once using the standard
//! library sort and once using Radix Sort. The data sets are sampled using an uniform
//! distribution.
//!
//! Keep in mind that the results may vary depending on the hardware, compiler version, operating
//! system version and configuration and the weather.
//!
//!
//! ### Small (1'000 elements)
//!
//! For small data sets Radix Sort can be an advantage for data types with up to 32 bits size. For
//! 64 bits, standard library sorting should be preferred.
//!
//! ```text
//! test unstable::bench_small_f32_rdxsort  ... bench:      25,629 ns/iter (+/- 399)
//! test unstable::bench_small_f32_std      ... bench:      76,913 ns/iter (+/- 670)
//! test unstable::bench_small_f64_rdxsort  ... bench:      54,663 ns/iter (+/- 1,589)
//! test unstable::bench_small_f64_std      ... bench:      81,153 ns/iter (+/- 1,325)
//! test unstable::bench_small_i16_rdxsort  ... bench:      13,173 ns/iter (+/- 119)
//! test unstable::bench_small_i16_std      ... bench:      32,490 ns/iter (+/- 547)
//! test unstable::bench_small_i32_rdxsort  ... bench:      25,470 ns/iter (+/- 302)
//! test unstable::bench_small_i32_std      ... bench:      30,539 ns/iter (+/- 1,021)
//! test unstable::bench_small_i64_rdxsort  ... bench:      52,444 ns/iter (+/- 3,399)
//! test unstable::bench_small_i64_std      ... bench:      30,512 ns/iter (+/- 277)
//! test unstable::bench_small_i8_rdxsort   ... bench:       8,075 ns/iter (+/- 968)
//! test unstable::bench_small_i8_std       ... bench:      43,994 ns/iter (+/- 1,099)
//! test unstable::bench_small_u16_rdxsort  ... bench:      16,006 ns/iter (+/- 587)
//! test unstable::bench_small_u16_std      ... bench:      33,313 ns/iter (+/- 2,342)
//! test unstable::bench_small_u32_rdxsort  ... bench:      30,923 ns/iter (+/- 1,738)
//! test unstable::bench_small_u32_std      ... bench:      34,286 ns/iter (+/- 3,338)
//! test unstable::bench_small_u64_rdxsort  ... bench:      65,384 ns/iter (+/- 7,461)
//! test unstable::bench_small_u64_std      ... bench:      34,208 ns/iter (+/- 717)
//! test unstable::bench_small_u8_rdxsort   ... bench:       8,367 ns/iter (+/- 784)
//! test unstable::bench_small_u8_std       ... bench:      45,695 ns/iter (+/- 1,256)
//! ```
//!
//!
//! ### Medium (10'000 elements)
//!
//! For medium data sets Radix Sort could be blindly used for all data types since the disadvantage
//! for types with 64 bits is quite small.
//!
//! ```text
//! test unstable::bench_medium_f32_rdxsort ... bench:     219,547 ns/iter (+/- 5,166)
//! test unstable::bench_medium_f32_std     ... bench:   1,126,436 ns/iter (+/- 13,894)
//! test unstable::bench_medium_f64_rdxsort ... bench:     426,906 ns/iter (+/- 7,456)
//! test unstable::bench_medium_f64_std     ... bench:   1,140,845 ns/iter (+/- 22,528)
//! test unstable::bench_medium_i16_rdxsort ... bench:     148,963 ns/iter (+/- 2,241)
//! test unstable::bench_medium_i16_std     ... bench:     470,194 ns/iter (+/- 9,461)
//! test unstable::bench_medium_i32_rdxsort ... bench:     234,601 ns/iter (+/- 2,066)
//! test unstable::bench_medium_i32_std     ... bench:     458,349 ns/iter (+/- 7,521)
//! test unstable::bench_medium_i64_rdxsort ... bench:     455,488 ns/iter (+/- 14,868)
//! test unstable::bench_medium_i64_std     ... bench:     466,938 ns/iter (+/- 37,052)
//! test unstable::bench_medium_i8_rdxsort  ... bench:      99,520 ns/iter (+/- 2,304)
//! test unstable::bench_medium_i8_std      ... bench:     623,255 ns/iter (+/- 10,640)
//! test unstable::bench_medium_u16_rdxsort ... bench:     141,764 ns/iter (+/- 2,476)
//! test unstable::bench_medium_u16_std     ... bench:     507,617 ns/iter (+/- 161,140)
//! test unstable::bench_medium_u32_rdxsort ... bench:     290,928 ns/iter (+/- 4,812)
//! test unstable::bench_medium_u32_std     ... bench:     508,403 ns/iter (+/- 8,650)
//! test unstable::bench_medium_u64_rdxsort ... bench:     578,533 ns/iter (+/- 12,939)
//! test unstable::bench_medium_u64_std     ... bench:     517,811 ns/iter (+/- 42,321)
//! test unstable::bench_medium_u8_rdxsort  ... bench:      75,994 ns/iter (+/- 3,254)
//! test unstable::bench_medium_u8_std      ... bench:     661,781 ns/iter (+/- 11,298)
//! ```
//!
//!
//! ### Large (100'000 elements)
//!
//! For large data sets, Radix Sort is great for all data types.
//!
//! ```text
//! test unstable::bench_large_f32_rdxsort  ... bench:   3,140,773 ns/iter (+/- 239,846)
//! test unstable::bench_large_f32_std      ... bench:  14,467,364 ns/iter (+/- 969,117)
//! test unstable::bench_large_f64_rdxsort  ... bench:   7,087,326 ns/iter (+/- 592,293)
//! test unstable::bench_large_f64_std      ... bench:  15,311,056 ns/iter (+/- 421,909)
//! test unstable::bench_large_i16_rdxsort  ... bench:   1,469,987 ns/iter (+/- 49,132)
//! test unstable::bench_large_i16_std      ... bench:   5,698,794 ns/iter (+/- 418,976)
//! test unstable::bench_large_i32_rdxsort  ... bench:   2,767,764 ns/iter (+/- 103,383)
//! test unstable::bench_large_i32_std      ... bench:   5,890,482 ns/iter (+/- 464,185)
//! test unstable::bench_large_i64_rdxsort  ... bench:   5,547,276 ns/iter (+/- 262,864)
//! test unstable::bench_large_i64_std      ... bench:   6,448,590 ns/iter (+/- 78,947)
//! test unstable::bench_large_i8_rdxsort   ... bench:   1,018,863 ns/iter (+/- 10,951)
//! test unstable::bench_large_i8_std       ... bench:   7,264,974 ns/iter (+/- 641,619)
//! test unstable::bench_large_u16_rdxsort  ... bench:   1,530,227 ns/iter (+/- 101,171)
//! test unstable::bench_large_u16_std      ... bench:   6,185,277 ns/iter (+/- 62,926)
//! test unstable::bench_large_u32_rdxsort  ... bench:   3,231,873 ns/iter (+/- 284,360)
//! test unstable::bench_large_u32_std      ... bench:   6,603,391 ns/iter (+/- 80,595)
//! test unstable::bench_large_u64_rdxsort  ... bench:   6,655,903 ns/iter (+/- 407,888)
//! test unstable::bench_large_u64_std      ... bench:   7,131,352 ns/iter (+/- 206,972)
//! test unstable::bench_large_u8_rdxsort   ... bench:     757,607 ns/iter (+/- 63,312)
//! test unstable::bench_large_u8_std       ... bench:   7,818,080 ns/iter (+/- 109,047)
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
