use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grids) = parse(input)?;

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for grid in grids {
        let value = grid[0][0];
        let lengths = grid
            .columns()
            .map(|v| v.fold(0, |acc, &x| if x == value { acc + 1 } else { acc }))
            .collect_vec();
        if value == '.' {
            keys.push(lengths);
        } else {
            locks.push(lengths);
        }
    }

    let mut res = 0;
    for (lock, key) in locks.into_iter().cartesian_product(keys) {
        if lock
            .into_iter()
            .zip(key)
            .all(|(a, b)| a <= b)
        {
            res += 1;
        }
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Vec<char>>>> {
    separated_list1(
        multispace1,
        separated_list1(line_ending, many1(one_of(".#"))),
    )(input)
}
