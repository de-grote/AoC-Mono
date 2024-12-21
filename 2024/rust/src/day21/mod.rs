#![allow(unstable_name_collisions)]

use crate::prelude::*;

type DirectionOrA = Option<Direction>;
#[derive(Debug, Clone, Copy, Default)]
enum Numpad {
    #[default]
    A,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}

pub fn part1(input: &str) -> Answer {
    let (_, sequences) = parse(input)?;

    let res = sequences
        .into_iter()
        .map(|sequence| {
            let numeric_code = sequence
                .iter()
                .filter_map(|&a| match a {
                    Numpad::A => None,
                    Numpad::K0 => Some('0'),
                    Numpad::K1 => Some('1'),
                    Numpad::K2 => Some('2'),
                    Numpad::K3 => Some('3'),
                    Numpad::K4 => Some('4'),
                    Numpad::K5 => Some('5'),
                    Numpad::K6 => Some('6'),
                    Numpad::K7 => Some('7'),
                    Numpad::K8 => Some('8'),
                    Numpad::K9 => Some('9'),
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let mut min = u64::MAX;
            let robot1_paths = paths_from_dir_to_numpad(&sequence);

            for path in robot1_paths
                .into_iter()
                .multi_cartesian_product()
                .map(|v| v.into_iter().flatten().collect_vec())
            {
                let robot2_paths = paths_from_dir_to_dir(&path, None);

                for path2 in robot2_paths
                    .into_iter()
                    .multi_cartesian_product()
                    .map(|v| v.into_iter().flatten().collect_vec())
                {
                    let robot3_paths = paths_from_dir_to_dir(&path2, None);

                    for path3 in robot3_paths
                        .into_iter()
                        .multi_cartesian_product()
                        .map(|v| v.into_iter().flatten().collect_vec())
                    {
                        min = min.min(path3.len() as u64);
                    }
                }
            }

            numeric_code * min
        })
        .sum::<u64>();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, sequences) = parse(input)?;

    let mut cache = HashMap::new();

    let res = sequences
        .into_iter()
        .map(|sequence| {
            let numeric_code = sequence
                .iter()
                .filter_map(|&a| match a {
                    Numpad::A => None,
                    Numpad::K0 => Some('0'),
                    Numpad::K1 => Some('1'),
                    Numpad::K2 => Some('2'),
                    Numpad::K3 => Some('3'),
                    Numpad::K4 => Some('4'),
                    Numpad::K5 => Some('5'),
                    Numpad::K6 => Some('6'),
                    Numpad::K7 => Some('7'),
                    Numpad::K8 => Some('8'),
                    Numpad::K9 => Some('9'),
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            let min =
                cost_to_press_button_sequnce(25, paths_from_dir_to_numpad(&sequence), &mut cache);

            numeric_code * min
        })
        .sum::<u64>();

    Ok(res.to_string())
}

fn cost_to_press_button(
    robot: u64,
    start: DirectionOrA,
    end: DirectionOrA,
    cache: &mut HashMap<(u64, DirectionOrA, DirectionOrA), u64>,
) -> u64 {
    if robot == 0 {
        return 1;
    }
    if let Some(&v) = cache.get(&(robot, start, end)) {
        return v;
    }
    let buttons = paths_from_dir_to_dir(&[end], start);
    let min = cost_to_press_button_sequnce(robot - 1, buttons, cache);
    cache.insert((robot, start, end), min);
    min
}

fn cost_to_press_button_sequnce(
    robot: u64,
    path: Vec<Vec<Vec<DirectionOrA>>>,
    cache: &mut HashMap<(u64, DirectionOrA, DirectionOrA), u64>,
) -> u64 {
    let mut min = u64::MAX;
    for a in path.into_iter().multi_cartesian_product().map(|v| {
        std::iter::once(None)
            .chain(v.into_iter().flatten())
            .tuple_windows::<(_, _)>()
            .collect_vec()
    }) {
        let mut acc = 0;
        for (s, e) in a {
            acc += cost_to_press_button(robot, s, e, cache);
        }
        min = min.min(acc);
    }
    min
}

fn paths_from_dir_to_numpad(path: &[Numpad]) -> Vec<Vec<Vec<DirectionOrA>>> {
    let mut v = Vec::new();
    path.iter().fold(Numpad::A, |acc, &numpad| {
        v.push(paths_numpad(acc, numpad));
        v.push(vec![vec![None]]);

        numpad
    });
    v
}

fn paths_from_dir_to_dir(
    path: &[DirectionOrA],
    start: DirectionOrA,
) -> Vec<Vec<Vec<DirectionOrA>>> {
    let mut v = Vec::new();
    path.iter().fold(start, |acc, &dir| {
        let new_path = paths_directions(acc, dir);
        if !new_path.is_empty() {
            v.push(new_path);
        }
        v.push(vec![vec![None]]);
        dir
    });
    v
}

const fn pos_numpad(c: Numpad) -> IVec2 {
    match c {
        Numpad::A => IVec2::ZERO,
        Numpad::K0 => IVec2::new(-1, 0),
        Numpad::K1 => IVec2::new(-2, -1),
        Numpad::K2 => IVec2::new(-1, -1),
        Numpad::K3 => IVec2::new(0, -1),
        Numpad::K4 => IVec2::new(-2, -2),
        Numpad::K5 => IVec2::new(-1, -2),
        Numpad::K6 => IVec2::new(0, -2),
        Numpad::K7 => IVec2::new(-2, -3),
        Numpad::K8 => IVec2::new(-1, -3),
        Numpad::K9 => IVec2::new(0, -3),
    }
}

const fn pos_directions(d: DirectionOrA) -> IVec2 {
    match d {
        None => IVec2::ZERO,
        Some(Direction::Up) => IVec2::new(-1, 0),
        Some(Direction::Right) => IVec2::new(0, 1),
        Some(Direction::Down) => IVec2::new(-1, 1),
        Some(Direction::Left) => IVec2::new(-2, 1),
    }
}

fn paths_numpad(start: Numpad, end: Numpad) -> Vec<Vec<DirectionOrA>> {
    let start_pos = pos_numpad(start);
    let end_pos = pos_numpad(end);
    paths(start_pos, end_pos, IVec2::new(-2, 0))
}

fn paths_directions(start: DirectionOrA, end: DirectionOrA) -> Vec<Vec<DirectionOrA>> {
    let start_pos = pos_directions(start);
    let end_pos = pos_directions(end);
    paths(start_pos, end_pos, IVec2::new(-2, 0))
}

fn paths(start: IVec2, end: IVec2, corner: IVec2) -> Vec<Vec<DirectionOrA>> {
    let delta = end - start;
    let y_step = match delta.y.cmp(&0) {
        Ordering::Less => vec![(Direction::Up, -delta.y)],
        Ordering::Equal => Vec::new(),
        Ordering::Greater => vec![(Direction::Down, delta.y)],
    };
    let x_step = match delta.x.cmp(&0) {
        Ordering::Less => vec![(Direction::Left, -delta.x)],
        Ordering::Equal => Vec::new(),
        Ordering::Greater => vec![(Direction::Right, delta.x)],
    };

    [
        [x_step.clone(), y_step.clone()].concat(),
        [y_step, x_step].concat(),
    ]
    .into_iter()
    .filter(|v| {
        let Some((dir, vel)) = v.first() else {
            return false;
        };
        dir.unit_vector() * vel + start != corner
    })
    .unique()
    .map(|v| {
        v.into_iter()
            .flat_map(|(dir, amount)| std::iter::repeat_n(Some(dir), amount as usize))
            .collect_vec()
    })
    .collect_vec()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Numpad>>> {
    separated_list1(
        line_ending,
        many1(one_of("A1234567890").map(|c| match c {
            'A' => Numpad::A,
            '0' => Numpad::K0,
            '1' => Numpad::K1,
            '2' => Numpad::K2,
            '3' => Numpad::K3,
            '4' => Numpad::K4,
            '5' => Numpad::K5,
            '6' => Numpad::K6,
            '7' => Numpad::K7,
            '8' => Numpad::K8,
            '9' => Numpad::K9,
            _ => unreachable!(),
        })),
    )(input)
}
