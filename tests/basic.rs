#![feature(phase)]

#[phase(plugin)]
extern crate speculate;

speculate! {
    it "works!" {
        assert_eq!(0u, 0);
    }

    it "works!!" {
        assert_eq!(1u, 1);
    }

    it "works!!!" {
        assert_eq!(2u, 2);
    }
}
