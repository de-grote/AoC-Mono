macro_rules! gen_test {
    // only part 1
    ($day:ident, $input:literal, $part1:literal) => {
        mod $day {
            use super::*;
            use crate::$day::*;

            #[test]
            fn test_part1() {
                assert_eq!($day::part1($input).unwrap().to_string(), $part1);
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
                assert_eq!($day::part1($input).unwrap().to_string(), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!($day::part2($input2).unwrap().to_string(), $part2);
            }
        }
    };
}

gen_test!(
    day01,
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
    "3",
    "6"
);
gen_test!(
    day02,
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    "1227775554",
    "4174379265"
);
gen_test!(
    day03,
    "987654321111111
811111111111119
234234234234278
818181911112111",
    "357",
    "3121910778619"
);
gen_test!(day04, "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.", "13", "43");
