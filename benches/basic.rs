#![feature(phase)]

#[phase(plugin)]
extern crate speculate;

extern crate test;

speculate! {
    bench "noop" |b| {
        b.iter(|| ());
    }

    describe "xor" {
        context "0 to 1000" {
            before {
                let limit = 1000u;
            }

            bench "using `fold`" |b| {
                b.iter(|| range(0, limit).fold(0, |a, b| a ^ b));
            }

            bench "using `for`" |b| {
                b.iter(|| {
                    let mut ret = 0;
                    for i in range(0u, limit) {
                        ret ^= i;
                    }
                    ret
                });
            }

            after {
                assert_eq!(limit, 1000);
            }
        }
    }
}
