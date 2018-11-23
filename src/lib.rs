//! `speculate` is a crate that provides a very simple macro
//! that is used to easily and elegantly define unit tests in Rust.
//! 
//! Please see the documentation for the [`speculate`](./fn.speculate.html) macro
//! for more information and examples.

#![feature(proc_macro_span, rustc_private)]
extern crate proc_macro;
extern crate proc_macro2;

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod block;
mod generator;

use block::Root;
use generator::Generate;

use proc_macro::TokenStream;


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
///   # #![feature(use_extern_macros, proc_macro_hygiene)]
///   # extern crate speculate;
///   # use speculate::speculate;
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
///   # #![feature(use_extern_macros, proc_macro_hygiene)]
///   # extern crate speculate;
///   # use speculate::speculate;
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
///   # #![feature(use_extern_macros, proc_macro_hygiene)]
///   # extern crate speculate;
///   # use speculate::speculate;
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
/// # #![feature(use_extern_macros, proc_macro_hygiene)]
/// # extern crate speculate;
/// # use speculate::speculate;
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
    let mut root = syn::parse2::<Root>(input).unwrap();

    let start_line = proc_macro::Span::call_site().start().line;
    let module_name = format!("speculate_{}", start_line);

    root.0.name = syn::Ident::new(&module_name, proc_macro2::Span::call_site());

    let mut prefix = quote!( #[allow(non_snake_case)] );
    let modl = root.0.generate(None);

    prefix.extend(modl);
    prefix.into()
}
