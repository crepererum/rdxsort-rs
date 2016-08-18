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
//! - **bool:** native implementation
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
//! shows the runtime in ns required for sorting data sets of different sizes. The data sets are
//! sampled using an uniform distribution. The best algorithm out of the following is marked:
//!
//! - [quicksort](https://crates.io/crates/quicksort)
//! - rdxsort (this crate)
//! - [standard library](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by)
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
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `4,342` | **`3,755`** | `28,237` |
//! | `f32` | `83,209` | **`29,585`** | `83,320` |
//! | `f64` | `83,648` | **`59,668`** | `85,491` |
//! | `i16` | `36,975` | **`13,731`** | `33,301` |
//! | `i32` | `34,101` | **`29,106`** | `33,205` |
//! | `i64` | `34,852` | `53,478` | **`33,818`** |
//! | `i8` | `27,015` | **`8,525`** | `47,286` |
//! | `u16` | `36,651` | **`17,278`** | `36,539` |
//! | `u32` | **`33,918`** | `34,077` | `36,100` |
//! | `u64` | **`35,677`** | `68,056` | `37,076` |
//! | `u8` | `26,797` | **`8,891`** | `49,235` |
//!
//!
//! ### Medium (10'000 elements)
//!
//! For medium data sets Radix Sort could be blindly used for all data types since the disadvantage
//! for types with 64 bits is quite small.
//!
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `57,909` | **`34,093`** | `480,016` |
//! | `f32` | `1,114,788` | **`250,939`** | `1,214,559` |
//! | `f64` | `1,128,267` | **`447,993`** | `1,206,347` |
//! | `i16` | `716,031` | **`153,701`** | `477,024` |
//! | `i32` | `704,039` | **`253,393`** | `497,010` |
//! | `i64` | `713,473` | **`487,089`** | `506,578` |
//! | `i8` | `418,997` | **`106,087`** | `669,052` |
//! | `u16` | `730,203` | **`152,142`** | `566,722` |
//! | `u32` | `935,165` | **`297,444`** | `549,351` |
//! | `u64` | `715,719` | `593,035` | **`559,335`** |
//! | `u8` | `418,612` | **`82,015`** | `710,016` |
//!
//!
//! ### Large (100'000 elements)
//!
//! For large data sets, Radix Sort is great for all data types.
//!
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `858,172` | **`360,872`** | `5,185,792` |
//! | `f32` | `14,123,860` | **`3,524,334`** | `15,728,264` |
//! | `f64` | `14,276,728` | **`7,567,796`** | `16,278,204` |
//! | `i16` | `8,953,393` | **`1,556,865`** | `5,952,820` |
//! | `i32` | `8,748,903` | **`2,758,901`** | `6,119,272` |
//! | `i64` | `8,751,121` | **`5,938,095`** | `7,047,055` |
//! | `i8` | `3,916,223` | **`1,091,063`** | `7,685,881` |
//! | `u16` | `8,946,741` | **`1,596,501`** | `6,703,832` |
//! | `u32` | `8,397,536` | **`3,286,443`** | `6,796,391` |
//! | `u64` | `8,711,661` | **`6,727,896`** | `7,912,789` |
//! | `u8` | `3,897,908` | **`787,374`** | `8,002,818` |
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
mod bool;
mod unsigned_integer;
mod signed_integer;
mod floats;

pub use template::RdxSortTemplate;
