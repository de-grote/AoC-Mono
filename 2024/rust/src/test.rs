macro_rules! gen_test {
    // only part 1
    ($day:ident, $input:literal, $part1:literal) => {
        mod $day {
            use super::*;
            use crate::$day::*;

            #[test]
            fn test_part1() {
                assert_eq!($day::part1($input).unwrap(), $part1);
            }
        }
    };
    // part 1 & 2 same input
    ($day:ident, $input:literal, $part1:literal, $part2:literal) => {
        gen_test!($day, $input, $input, $part1, $part2);
    };
    // part 1 & 2 different input
    ($day:ident, $input:literal, $input2:literal, $part1:literal, $part2:literal) => {
        mod $day {
            use super::*;
            use crate::$day::*;

            #[test]
            fn test_part1() {
                assert_eq!($day::part1($input).unwrap(), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!($day::part2($input2).unwrap(), $part2);
            }
        }
    };
}

gen_test!(
    day01,
    "3   4
4   3
2   5
1   3
3   9
3   3",
    "11",
    "31"
);
gen_test!(
    day02,
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    "2",
    "4"
);
