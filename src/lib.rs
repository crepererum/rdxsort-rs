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
//! - **char:** native implementation
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
//! | `bool` | `4,199` | **`3,579`** | `27,786` |
//! | `char` | `33,897` | **`32,842`** | `35,840` |
//! | `f32` | `82,225` | **`28,867`** | `82,526` |
//! | `f64` | `88,772` | **`60,041`** | `85,664` |
//! | `i16` | `34,817` | **`15,149`** | `33,171` |
//! | `i32` | `34,192` | **`27,006`** | `31,786` |
//! | `i64` | `36,417` | `54,626` | **`33,133`** |
//! | `i8` | `26,135` | **`8,535`** | `47,711` |
//! | `u16` | `35,892` | **`16,160`** | `34,442` |
//! | `u32` | `33,380` | **`32,779`** | `34,081` |
//! | `u64` | `35,047` | `69,763` | **`34,880`** |
//! | `u8` | `26,974` | **`9,127`** | `48,248` |
//!
//!
//! ### Medium (10'000 elements)
//!
//! For medium data sets Radix Sort could be blindly used for all data types since the disadvantage
//! for types with 64 bits is quite small.
//!
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `57,110` | **`33,837`** | `424,807` |
//! | `char` | `701,575` | **`300,163`** | `533,560` |
//! | `f32` | `1,144,874` | **`249,719`** | `1,178,341` |
//! | `f64` | `1,164,848` | **`452,096`** | `1,223,874` |
//! | `i16` | `697,117` | **`151,539`** | `479,502` |
//! | `i32` | `697,657` | **`256,532`** | `480,306` |
//! | `i64` | `704,445` | **`472,059`** | `498,994` |
//! | `i8` | `398,733` | **`103,897`** | `653,829` |
//! | `u16` | `709,508` | **`144,522`** | `529,706` |
//! | `u32` | `704,948` | **`310,517`** | `533,432` |
//! | `u64` | `703,639` | `582,066` | **`550,873`** |
//! | `u8` | `407,994` | **`77,790`** | `683,931` |
//!
//!
//! ### Large (100'000 elements)
//!
//! For large data sets, Radix Sort is great for all data types.
//!
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `872,380` | **`352,250`** | `5,273,593` |
//! | `char` | `8,718,712` | **`3,674,520`** | `6,808,558` |
//! | `f32` | `14,525,882` | **`3,602,960`** | `14,794,566` |
//! | `f64` | `14,388,793` | **`7,507,376`** | `16,555,355` |
//! | `i16` | `8,839,427` | **`1,552,497`** | `5,977,884` |
//! | `i32` | `8,729,181` | **`2,784,668`** | `6,371,906` |
//! | `i64` | `8,670,213` | **`5,962,087`** | `7,001,072` |
//! | `i8` | `3,823,044` | **`1,063,651`** | `7,584,821` |
//! | `u16` | `8,899,513` | **`1,521,481`** | `6,715,678` |
//! | `u32` | `8,497,530` | **`3,407,685`** | `6,751,882` |
//! | `u64` | `8,560,613` | **`6,788,488`** | `7,689,427` |
//! | `u8` | `3,918,831` | **`775,895`** | `8,283,952` |
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
mod char;
mod unsigned_integer;
mod signed_integer;
mod floats;

pub use template::RdxSortTemplate;
