use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let mut set = HashSet::new();
            let mut next_set = HashSet::new();
            set.insert(IVec2::new(x as i32, y as i32));
            for n in 0..9 {
                for item in set {
                    if grid.at(item) == Some(&n) {
                        next_set.extend(orthogonal_points(item));
                    }
                }
                set = next_set;
                next_set = HashSet::new();
            }
            res += set
                .into_iter()
                .filter(|&pos| grid.at(pos) == Some(&9))
                .count();
        }
    }

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let mut map = HashMap::new();
            let mut next_map = HashMap::new();
            map.insert(IVec2::new(x as i32, y as i32), 1);
            for n in 0..9 {
                for (item, paths) in map {
                    if grid.at(item) == Some(&n) {
                        for point in orthogonal_points(item) {
                            *next_map.entry(point).or_default() += paths
                        }
                    }
                }
                map = next_map;
                next_map = HashMap::new();
            }
            res += map
                .into_iter()
                .filter_map(|(pos, paths)| (grid.at(pos) == Some(&9)).then_some(paths))
                .sum::<i32>();
        }
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(
        line_ending,
        many1(satisfy(|c| c.is_ascii_digit()).map(|d| d.to_digit(10).unwrap() as u8)),
    )(input)
}
