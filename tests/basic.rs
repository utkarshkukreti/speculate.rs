#![feature(phase)]

#[phase(plugin)]
extern crate speculate;

speculate! {
    before {
    }

    after {
    }

    it "works at level 1!" {
        assert_eq!(0u, 0);
    }

    describe "something" {
        before {
        }

        describe "nested" {
            it "works at level 3!" {
                assert_eq!(1u, 1);
            }

            after {
            }
        }
    }
}
