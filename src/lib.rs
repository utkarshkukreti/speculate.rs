//! `speculate` is a crate that provides a very simple macro
//! that is used to easily and elegantly define unit tests in Rust.
//!
//! Please see the documentation for the [`speculate`](./macro.speculate.html) macro
//! for more information and examples.

extern crate proc_macro;

use crate::{block::Describe, generator::Generate};
use proc_macro::TokenStream;

mod block;
mod generator;

/// Creates a `test` module using a friendly syntax.
///
/// Inside this block, the following elements can be used:
///
/// * `describe` (or its alias `context`) - to group tests in a hierarchy, for
///   readability. Can be arbitrarily nested.
///
/// * `before` - contains setup code that's inserted before every sibling and nested
///   `it` and `bench` blocks.
///
/// * `after` - contains teardown code that's inserted after every sibling and
///   nested `it` and `bench` blocks.
///
/// * `it` (or its alias `test`) - contains tests.
///
///   For example:
///
///   ```rust
///   #[macro_use] extern crate speculate as other_speculate;
///   # fn main() {}
///   # speculate! {
///   it "can add 1 and 2" {
///       assert_eq!(1 + 2, 3);
///   }
///   # }
///   ```
///
///   You can optionally add attributes to this block:
///
///   ```rust
///   #[macro_use] extern crate speculate as other_speculate;
///   # fn main() {}
///   # speculate! {
///   #[ignore]
///   test "ignore" {
///       assert_eq!(1, 2);
///   }
///
///   #[should_panic]
///   test "should panic" {
///       assert_eq!(1, 2);
///   }
///
///   #[should_panic(expected = "foo")]
///   test "should panic with foo" {
///       panic!("foo");
///   }
///   # }
///   ```
///
/// * `bench` - contains benchmarks (using [`Bencher`](https://doc.rust-lang.org/test/struct.Bencher.html)).
///
///   For example:
///
///   ```rust
///   #[macro_use] extern crate speculate as other_speculate;
///   # fn main() {}
///   # speculate! {
///   bench "xor 1 to 1000" |b| {
///       // Here, `b` is a `test::Bencher`.
///       b.iter(|| (0..1000).fold(0, |a, b| a ^ b));
///   }
///   # }
///   ```
///
/// * Any other Rust "Item", such as `static`, `const`, `fn`, etc.
///
/// # Example
///
/// ```rust
///   #[macro_use] extern crate speculate as other_speculate;
///   # fn main() {}
/// speculate! {
///     const ZERO: i32 = 0;
///
///     fn add(a: i32, b: i32) -> i32 {
///         a + b
///     }
///
///     describe "math" {
///         const ONE: i32 = 1;
///
///         fn sub(a: i32, b: i32) -> i32 {
///             a - b
///         }
///
///         before {
///             let two = ONE + ONE;
///         }
///
///         it "can add stuff" {
///             assert_eq!(ONE, add(ZERO, ONE));
///             assert_eq!(two, add(ONE, ONE));
///         }
///
///         it "can subtract stuff" {
///             assert_eq!(ZERO, sub(ONE, ONE));
///             assert_eq!(ONE, sub(two, ONE));
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn speculate(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    // println!("Speculate input tokens: {}", input.to_string());

    let input_describe: Describe = syn::parse2(input.clone()).unwrap();

    let output: proc_macro2::TokenStream = input_describe.generate(None);
    // println!("Generated output tokens: {}", output.to_string());

    output.into()
}
