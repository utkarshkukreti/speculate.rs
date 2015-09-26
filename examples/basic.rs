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
