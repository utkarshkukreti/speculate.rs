#![feature(proc_macro_hygiene, test)]
extern crate test;

use speculate::speculate;

speculate! {
    bench "noop" |b| {
        b.iter(|| ());
    }

    describe "xor" {
        context "0 to 1000" {
            before {
                let limit = 1000u32;
            }

            bench "using `fold`" |b| {
                b.iter(|| (0..limit).fold(0, |a, b| a ^ b));
            }

            bench "using `for`" |b| {
                b.iter(|| {
                    let mut ret = 0;
                    for i in 0..limit {
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
