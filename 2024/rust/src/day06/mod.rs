use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;

    let mut area = HashSet::new();
    let mut position = IVec2::ZERO;
    let mut direction = Direction::Up;
    let (width, height) = grid.size();

    for (y, row) in grid.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            match v {
                '#' => {
                    area.insert(IVec2::new(x as i32, y as i32));
                }
                '>' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Right
                }
                '<' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Left
                }
                '^' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Up
                }
                'v' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Down
                }
                _ => {}
            }
        }
    }

    let mut been = HashSet::new();
    while (0..(width as i32)).contains(&position.x) && (0..(height as i32)).contains(&position.y) {
        been.insert(position);
        let next = position + direction.unit_vector();
        if area.contains(&next) {
            direction = direction.rotate_right();
        } else {
            position = next;
        }
    }

    Ok(been.len().to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;

    let mut area = HashSet::new();
    let mut position = IVec2::ZERO;
    let mut direction = Direction::Up;
    let (width, height) = grid.size();

    for (y, row) in grid.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            match v {
                '#' => {
                    area.insert(IVec2::new(x as i32, y as i32));
                }
                '>' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Right
                }
                '<' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Left
                }
                '^' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Up
                }
                'v' => {
                    position = IVec2::new(x as i32, y as i32);
                    direction = Direction::Down
                }
                _ => {}
            }
        }
    }

    let start_pos = position;
    let start_dir = direction;

    let mut been = HashSet::new();
    while (0..(width as i32)).contains(&position.x) && (0..(height as i32)).contains(&position.y) {
        been.insert(position);
        let next = position + direction.unit_vector();
        if area.contains(&next) {
            direction = direction.rotate_right();
        } else {
            position = next;
        }
    }

    let mut res: u32 = 0;
    // in my orginal solution i looped over all positions here and didn't calculate been
    // but this is about 2-4x faster
    for pos in been {
        if pos == start_pos || !area.insert(pos) {
            continue;
        }
        position = start_pos;
        direction = start_dir;
        let mut been = HashSet::new();
        while (0..(width as i32)).contains(&position.x)
            && (0..(height as i32)).contains(&position.y)
        {
            if !been.insert((position, direction)) {
                res += 1;
                break;
            };
            let next = position + direction.unit_vector();
            if area.contains(&next) {
                direction = direction.rotate_right();
            } else {
                position = next;
            }
        }
        area.remove(&pos);
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of(".#v^<>")))(input)
}
