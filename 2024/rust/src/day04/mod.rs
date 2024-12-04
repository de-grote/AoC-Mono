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
                    get_pos(&grid, pos),
                    get_pos(&grid, pos + dp),
                    get_pos(&grid, pos + dp * 2),
                    get_pos(&grid, pos + dp * 3),
                ) {
                    res += 1;
                }
            }
        }
    }

    Ok(res.to_string())
}

fn get_pos(v: &[Vec<char>], pos: IVec2) -> Option<char> {
    v.get(pos.y as usize)
        .and_then(|y2| y2.get(pos.x as usize).copied())
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
                    get_pos(&grid, pos),
                    get_pos(&grid, pos + dp),
                    get_pos(&grid, pos + dp.with_y(-dp.y)),
                    get_pos(&grid, pos + dp.with_x(-dp.x)),
                    get_pos(&grid, pos - dp),
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
