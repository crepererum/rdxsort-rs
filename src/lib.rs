//! # RdxSort
//!
//! This crate implements [Radix Sort](https://en.wikipedia.org/wiki/Radix_sort) for slices of
//! different data types, either directly or by exploiting the implementation of other data types.
//!
//! The main reason for implementing yet another sorting algorithm is that most sorting algorithms
//! are comparative methods. To sort data, they rely on a function that compares data elements. It
//! can be proven that this leads to a runtime complexity of `O(n*log(n))` in average and `O(n^2)`
//! in the worst case. In contrast to that, Radix Sort exploits that fact that many data types have
//! a limited range of possible values and within that range a limited resolution (that also holds
//! for floating point numbers). For a detailed explanation see the
//! [Wikipedia article](https://en.wikipedia.org/wiki/Radix_sort). The result of this special
//! treatment is a lowered and constant complexity of `O(n*k)` where `k` is the number of fixed
//! rounds required to sort a specific data type.
//!
//!
//! ## Supported Data Types
//!
//! Currently, the following data types are supported:
//!
//! - **bool:** simple split into 2 junks
//! - **char:** behaves like `u32`
//! - **unsigned integers:** native implementation, depending on the width
//! - **signed integers:** splitting into positive and negative parts and using the unsigned
//!   implementation
//! - **floats:** splitting of data into positive and negative numbers and treating the two
//!   ranges as unsigned integer values. `NaN`s are not supported!
//! - **arrays, tuples:** use the implementation of the inner data types
//! - *custom data types...: fill in the provided template trait*
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
//! integers benefit the most, long types do not show that huge improvements. The following listing
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
//! | `bool` | `4,070` | **`2,246`** | `26,068` |
//! | `char` | `31,121` | **`20,204`** | `34,051` |
//! | `f32` | `79,714` | **`25,825`** | `77,774` |
//! | `f64` | `80,954` | **`52,262`** | `79,431` |
//! | `i16` | `32,896` | **`12,496`** | `31,167` |
//! | `i32` | `32,854` | **`22,009`** | `30,713` |
//! | `i64` | `33,064` | `53,366` | **`31,669`** |
//! | `i8` | `24,819` | **`8,190`** | `46,281` |
//! | `u16` | `35,252` | **`9,937`** | `33,946` |
//! | `u32` | `33,002` | **`19,202`** | `33,627` |
//! | `u64` | **`32,986`** | `47,739` | `33,204` |
//! | `u8` | `25,425` | **`5,170`** | `47,369` |
//!
//!
//! ### Medium (10'000 elements)
//!
//! For medium data sets Radix Sort could be blindly used for all data types since the disadvantage
//! for types with 64 bits is quite small.
//!
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `52,211` | **`22,083`** | `400,111` |
//! | `char` | `655,553` | **`192,328`** | `508,557` |
//! | `f32` | `1,086,882` | **`230,670`** | `1,117,565` |
//! | `f64` | `1,095,529` | **`417,104`** | `1,152,340` |
//! | `i16` | `665,131` | **`108,128`** | `455,047` |
//! | `i32` | `650,501` | **`202,533`** | `460,097` |
//! | `i64` | `669,643` | **`378,480`** | `470,572` |
//! | `i8` | `383,545` | **`65,521`** | `617,405` |
//! | `u16` | `675,060` | **`78,424`** | `508,264` |
//! | `u32` | `670,348` | **`177,068`** | `511,134` |
//! | `u64` | `664,549` | **`342,935`** | `517,176` |
//! | `u8` | `386,572` | **`37,012`** | `655,377` |
//!
//!
//! ### Large (100'000 elements)
//!
//! For large data sets, Radix Sort is great for all data types.
//!
//! | type | quicksort | rdxsort | std |
//! |-----:|----------:|--------:|----:|
//! | `bool` | `815,653` | **`216,809`** | `4,900,426` |
//! | `char` | `8,131,402` | **`2,538,064`** | `6,333,865` |
//! | `f32` | `13,554,291` | **`3,264,657`** | `14,340,082` |
//! | `f64` | `13,746,190` | **`7,122,334`** | `15,563,206` |
//! | `i16` | `8,235,642` | **`1,248,289`** | `5,575,196` |
//! | `i32` | `8,184,902` | **`2,494,882`** | `5,852,913` |
//! | `i64` | `8,222,482` | **`5,446,507`** | `6,561,292` |
//! | `i8` | `3,664,532` | **`703,288`** | `7,118,816` |
//! | `u16` | `8,272,903` | **`866,833`** | `6,291,101` |
//! | `u32` | `8,193,408` | **`2,051,413`** | `6,395,163` |
//! | `u64` | `8,179,078` | **`4,393,579`** | `7,216,868` |
//! | `u8` | `3,681,361` | **`367,240`** | `7,816,775` |
//!
//!
//! ## Implementing New Types
//!
//! This crate enables you to add support for new types by implementing `Rdx`. It
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
//! impl Rdx for Foo {
//!     // using `#[inline]` is generally recommended since it helps
//!     // the compiler to optimize the sorting algorithm
//!     #[inline]
//!     fn cfg_nbuckets() -> usize {
//!         // usually too high, but works as a simple demonstration
//!         // `256 = 2^8`
//!         256
//!     }
//!
//!     #[inline]
//!     fn cfg_nrounds() -> usize {
//!         // one per sub-type
//!         2
//!     }
//!
//!     #[inline]
//!     fn get_bucket(&self, round: usize) -> usize {
//!         // return the least significant digit first
//!         if round == 0 {
//!             self.b as usize
//!         } else {
//!             self.a as usize
//!         }
//!     }
//!
//!     #[inline]
//!     fn reverse(_round: usize, _bucket: usize) -> bool {
//!         // not required in our case
//!         false
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

/// Generic Radix Sort implementation
///
/// Works by splitting the work in rounds. During every round, the data is sorted into buckets and
/// is then collected again. Rounds are count from `0` to (exclusive) `cfg_nrounds()`.
///
/// The number of buckets is fixed for all rounds. That does not mean that the template has to use
/// all of them, since unused buckets are just empty and have no effect on the collected results.
/// Same hold for the number of rounds. Over-booking one or both of them wastes resources of
/// course.
///
/// **WARNING: The result returned from `get_bucket()` is not checked. Wrong implementations may
/// crash the program, or destroy the world, or both!!!**
pub trait Rdx {
    /// Sets the number of buckets used by the generic implementation.
    fn cfg_nbuckets() -> usize;

    /// Sets the number of rounds scheduled by the generic implementation.
    fn cfg_nrounds() -> usize;

    /// Returns the bucket, depending on the round.
    ///
    /// This should respect the radix, e.g.:
    ///
    /// - if the number of buckets is `2` and the type is an unsigned integer, then the result is
    ///   the bit starting with the least significant one.
    /// - if the number of buckets is `8` and the type is an unsigned integer, then the result is
    ///   the byte starting with the least significant one.
    ///
    /// **Never** return a bucker greater or equal the number of buckets. See warning above!
    fn get_bucket(&self, round: usize) -> usize;

    /// Describes the fact that the content of a bucket should be copied back in reverse order
    /// after a certain round.
    fn reverse(round: usize, bucket: usize) -> bool;
}

/// Implements `t1` as alias of `t2`, e.g. `usize = u64` on platforms that have 64 bit pointers.
#[macro_export]
macro_rules! rdx_alias {
    ($t1:ty = $t2:ty) => {
        impl Rdx for $t1 {
            #[inline]
            fn cfg_nbuckets() -> usize {
                <$t2 as Rdx>::cfg_nbuckets()
            }

            #[inline]
            fn cfg_nrounds() -> usize {
                <$t2 as Rdx>::cfg_nrounds()
            }

            #[inline]
            fn get_bucket(&self, round: usize) -> usize {
                (*self as $t2).get_bucket(round)
            }


            #[inline]
            fn reverse(round: usize, bucket: usize) -> bool {
                <$t2 as Rdx>::reverse(round, bucket)
            }
        }
    }
}

mod sort;
mod tree;
mod types;

pub use sort::RdxSort;
pub use tree::{RdxTree, RdxTreeIter};
