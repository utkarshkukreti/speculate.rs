extern crate speculate;
use speculate::speculate;

struct SomeStruct(usize);

speculate! {
    describe "parameterized tests" {

        it "single parameter and single test case" for (even_number: usize) [
            "2" => (2)
        ] {
            assert_eq!(even_number % 2 == 0, true);
        }

        it "many parameters, many test cases" for (a: usize, b: usize, expected: usize) [
            "1 and 2" => (1, 2, 3),
            "2 and 2" => (2, 2, 4),
        ] {
            assert_eq!(a + b, expected);
        }

        it "works with non-primitive types" for (a: SomeStruct, b: SomeStruct, expected: usize) [
            "SomeStruct(1) + SomeStruct(2)" => (SomeStruct(1), SomeStruct(2), 3),
            "SomeStruct(2) + SomeStruct(2)" => (SomeStruct(2), SomeStruct(2), 4)
        ] {
            assert_eq!(a.0 + b.0, expected);
        }

        it "works with macros" for (mut v: Vec<usize>, to_push: usize, expected: Vec<usize>) [
            "some number" => (vec![1, 24], 2, vec![1, 24, 2]),
            "another number" => (vec![3, 15], 11, vec![3, 15, 11])
        ] {
            v.push(to_push);
            assert_eq!(v, expected);
        }

        test "works with test keyword instead of it keyword" for (a: usize) [
            "number equals 2" => (2)
        ] {
            assert_eq!(a, 2);
        }
    }
}
