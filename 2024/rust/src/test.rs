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
gen_test!(
    day03,
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    "161",
    "48"
);
gen_test!(
    day04,
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
    "18",
    "9"
);
gen_test!(
    day05,
    "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
    "143",
    "123"
);
gen_test!(
    day06,
    "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    "41",
    "6"
);
gen_test!(
    day07,
    "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    "3749",
    "11387"
);
gen_test!(
    day08,
    "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    "14",
    "34"
);
gen_test!(day09, "2333133121414131402", "1928", "2858");
gen_test!(
    day10,
    "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    "36",
    "81"
);
gen_test!(day11, "125 17", "55312");
