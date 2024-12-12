use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut done = HashSet::new();
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            if done.contains(&pos) {
                continue;
            }
            let mut perimeter = 0;
            let mut seen = HashSet::new();
            let mut current = HashSet::new();
            current.insert(pos);
            while let Some(&p) = current.iter().next() {
                current.remove(&p);
                seen.insert(p);
                for n in orthogonal_points(p) {
                    if grid.at(n) == Some(v) {
                        if !seen.contains(&n) {
                            current.insert(n);
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }
            res += seen.len() as u32 * perimeter;
            done.extend(seen);
        }
    }

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut done = HashSet::new();
    let mut res = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            if done.contains(&pos) {
                continue;
            }
            let mut seen = HashSet::new();
            let mut current = HashSet::new();
            let mut walls = HashSet::new();
            current.insert(pos);
            while let Some(&p) = current.iter().next() {
                current.remove(&p);
                seen.insert(p);
                for dp in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let n = p + dp.unit_vector();
                    if grid.at(n) == Some(v) {
                        if !seen.contains(&n) {
                            current.insert(n);
                        }
                    } else {
                        walls.insert((n, dp));
                    }
                }
            }
            let mut sides = 0;
            while let Some(&(p, dir)) = walls.iter().next() {
                walls.remove(&(p, dir));
                sides += 1;
                let right = dir.rotate_right().unit_vector();
                let mut r = p + right;
                while walls.remove(&(r, dir)) {
                    r += right;
                }
                let left = dir.rotate_left().unit_vector();
                let mut l = p + left;
                while walls.remove(&(l, dir)) {
                    l += left;
                }
            }
            res += seen.len() as u32 * sides;
            done.extend(seen);
        }
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(satisfy(|c| !c.is_whitespace())))(input)
}
