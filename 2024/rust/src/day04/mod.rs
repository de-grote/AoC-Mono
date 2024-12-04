use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for dp in adjacent_points(IVec2::ZERO) {
                let pos = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                if let (Some('X'), Some('M'), Some('A'), Some('S')) = (
                    grid.at(pos),
                    grid.at(pos + dp),
                    grid.at(pos + dp * 2),
                    grid.at(pos + dp * 3),
                ) {
                    res += 1;
                }
            }
        }
    }

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for dp in diagonal_points(IVec2::ZERO) {
                let pos = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                let matches = (
                    grid.at(pos),
                    grid.at(pos + dp),
                    grid.at(pos + dp.with_y(-dp.y)),
                    grid.at(pos + dp.with_x(-dp.x)),
                    grid.at(pos - dp),
                );
                if let (Some('A'), Some('S'), Some('M'), Some('S'), Some('M')) = matches {
                    res += 1;
                    break;
                }
                if let (Some('A'), Some('S'), Some('S'), Some('M'), Some('M')) = matches {
                    res += 1;
                    break;
                }
            }
        }
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of("XMAS.")))(input)
}
