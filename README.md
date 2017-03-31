# speculate.rs

> An RSpec inspired minimal testing framework for Rust.

Speculate uses a syntax extension to generate test functions from the DSL at
compile time, which unfortunately requires a nightly version of Rust. I
recommend using [multirust](https://github.com/brson/multirust) to easily
install and switch between stable and nightly versions of Rust.

## Installation

Add `speculate` to the `dev-dependencies` section of your `Cargo.toml`:

```toml
[dev-dependencies]
speculate = "0.0.24"
```

And add the following to the top of the Rust file you want to add tests for:

```rust
#![feature(plugin)]
#![plugin(speculate)]
```

## Usage

Speculate provides the `speculate!` syntax extension.
Inside `speculate! { ... }`, you can have any "Item", like `static`, `const`,
`fn`, etc, and 5 special types of blocks:

* `describe` (or its alias `context`) - to group tests in a hierarchy, for
  readability. Can be arbitrarily nested.

* `before` - contains setup code that's inserted before every sibling and nested
  `it` and `bench` blocks.

* `after` - contains teardown code that's inserted after every sibling and
  nested `it` and `bench` blocks.

* `it` (or its alias `test`) - contains tests.

  For example:

  ```rust
  it "can add 1 and 2" {
      assert_eq!(1 + 2, 3);
  }
  ```

* `bench` - contains benchmarks.

  For example:

  ```rust
  bench "xor 1 to 1000" |b| {
      b.iter(|| (0..1000).fold(0, |a, b| a ^ b));
  }
  ```

## Complete Example (from `tests/example.rs`)

```rust
#![feature(plugin)]
#![plugin(speculate)]

speculate! {
    const ZERO: i32 = 0;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    describe "math" {
        const ONE: i32 = 1;

        fn sub(a: i32, b: i32) -> i32 {
            a - b
        }

        before {
            let two = ONE + ONE;
        }

        it "can add stuff" {
            assert_eq!(ONE, add(ZERO, ONE));
            assert_eq!(two, add(ONE, ONE));
        }

        it "can subtract stuff" {
            assert_eq!(ZERO, sub(ONE, ONE));
            assert_eq!(ONE, sub(two, ONE));
        }
    }
}
```

## License

MIT

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
