#![allow(unstable)]
#![feature(plugin)]

#[plugin]
extern crate speculate;

pub fn zero() -> u32 {
    0
}

speculate! {
    before {
        let mut i = zero();
    }

    after {
        assert_eq!(i, 5);
    }

    it "works at level 1!" {
        assert_eq!(i, zero());
        i = 5;
    }

    describe "something" {
        before {
            assert_eq!(i, zero());
            i = 1;
        }

        it "works at level 2!" {
            assert_eq!(i, 1);
            i = 4;
        }

        after {
            assert_eq!(i, 4);
            i = 5;
        }

        context "nested" {
            before {
                assert_eq!(i, 1);
                i = 2;
            }

            it "works at level 3!" {
                assert_eq!(i, 2);
                i = 3;
            }

            after {
                assert_eq!(i, 3);
                i = 4;
            }
        }

        it "works at level 2 after context!" {
            assert_eq!(i, 1);
            i = 4;
        }
    }

    it "works at level 1 after describe!" {
        assert_eq!(i, zero());
        i = 5;
    }
}

// Parsing edge cases
mod ec1 {
    speculate! {
    }
}

mod ec2 {
    speculate! {
        before {}
    }
}

mod ec3 {
    speculate! {
        it "foo" {}
    }
}

mod ec4 {
    speculate! {
        after {}
    }
}

mod ec5 {
    speculate! {
        before {}
        it "foo" {}
        after {}
    }
}
