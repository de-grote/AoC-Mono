use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;

    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;

    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == 'S' {
                start = IVec2::new(x as i32, y as i32);
            } else if v == 'E' {
                end = IVec2::new(x as i32, y as i32);
            }
        }
    }

    let mut pos = start;
    let mut path = vec![];
    while pos != end {
        for p in orthogonal_points(pos) {
            if Some(&p) != path.last() && grid.at(p).is_some_and(|&c| c != '#') {
                path.push(pos);
                pos = p;
                break;
            }
        }
    }
    path.push(end);

    let delta = if cfg!(test) { 1 } else { 100 };

    let mut res = 0;
    for (i, &pos) in path.iter().enumerate() {
        res += path
            .iter()
            .skip(i + delta)
            .enumerate()
            .filter(|&(j, &p)| {
                let diff = pos - p;
                let distance = diff.abs().element_sum();
                distance <= 2 && distance <= j as i32
            })
            .count();
    }

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;

    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;

    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == 'S' {
                start = IVec2::new(x as i32, y as i32);
            } else if v == 'E' {
                end = IVec2::new(x as i32, y as i32);
            }
        }
    }

    let mut pos = start;
    let mut path = vec![];
    while pos != end {
        for p in orthogonal_points(pos) {
            if Some(&p) != path.last() && grid.at(p).is_some_and(|&c| c != '#') {
                path.push(pos);
                pos = p;
                break;
            }
        }
    }
    path.push(end);

    let delta = if cfg!(test) { 50 } else { 100 };

    let mut res = 0;
    for (i, &pos) in path.iter().enumerate() {
        res += path
            .iter()
            .skip(i + delta)
            .enumerate()
            .filter(|&(j, &p)| {
                let diff = pos - p;
                let distance = diff.abs().element_sum();
                distance <= 20 && distance <= j as i32
            })
            .count();
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(satisfy(|c| !c.is_whitespace())))(input)
}
