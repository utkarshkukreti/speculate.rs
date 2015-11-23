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
speculate = "0.0.17"
```

And add the following to the top of the Rust file you want to add tests for:

```rust
#![feature(plugin)]
#![plugin(speculate)]
```

## Usage

Speculate provides the `speculate!` syntax extension.
Inside `speculate! { ... }`, you can use 5 different types of blocks:

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

## Complete Example (from `examples/basic.rs`)

```rust
#![feature(plugin)]
#![plugin(speculate)]

pub mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

speculate! {
    describe "math" {
        before {
            let zero = 0;
            let one = 1;
        }

        it "can add stuff" {
            assert_eq!(one, ::math::add(zero, one));
        }

        it "can subtract stuff" {
            assert_eq!(zero, ::math::sub(one, one));
        }
    }
}

fn main() {
}
```

## License

MIT
