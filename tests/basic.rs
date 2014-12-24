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
                let i = 1u;
                assert_eq!(i, 1);
            }

            after {
            }
        }
    }
}
