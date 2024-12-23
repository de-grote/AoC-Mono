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
gen_test!(
    day11,
    "125 17",
    "55312",
    "65601038650482" // this isn't given
);
gen_test!(
    day12,
    "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    "1930",
    "1206"
);
gen_test!(
    day13,
    "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    "480",
    "875318608908" // this isn't given
);
gen_test!(
    day14,
    "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    "12" // no test for p2
);
gen_test!(
    day15,
    "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    "10092",
    "9021"
);
gen_test!(
    day16,
    "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
    "7036",
    "45"
);
gen_test!(
    day17,
    "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
    "4,6,3,5,6,3,5,2,1,0",
    "117440"
);
gen_test!(
    day18,
    "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    "22",
    "6,1"
);
gen_test!(
    day19,
    "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    "6",
    "16"
);
gen_test!(
    day20,
    "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
    "44",
    "285"
);
gen_test!(
    day21,
    "029A
980A
179A
456A
379A",
    "126384",
    "154115708116294" // this isn't given
);
gen_test!(
    day22,
    "1
10
100
2024",
    "37327623",
    "23"
);
gen_test!(
    day23,
    "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    "7",
    "co,de,ka,ta"
);
