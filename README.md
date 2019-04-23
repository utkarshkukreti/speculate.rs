# speculate.rs [![Build Status](https://travis-ci.org/utkarshkukreti/speculate.rs.svg?branch=master)](https://travis-ci.org/utkarshkukreti/speculate.rs) [![speculate at crates.io](https://img.shields.io/crates/v/speculate.svg)](https://crates.io/crates/speculate)

> An RSpec inspired minimal testing framework for Rust.

## Installation

Add `speculate` to the `dev-dependencies` section of your `Cargo.toml`:

```toml
[dev-dependencies]
speculate = "0.1"
```

And add the following to the top of the Rust file you want to add tests for:

```rust
#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;  // Must be imported into the current scope.
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

  You can optionally add attributes to this block:

  ```rust
  #[ignore]
  test "ignore" {
      assert_eq!(1, 2);
  }

  #[should_panic]
  test "should panic" {
      assert_eq!(1, 2);
  }

  #[should_panic(expected = "foo")]
  test "should panic with foo" {
      panic!("foo");
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
extern crate speculate;

use speculate::speculate;

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
