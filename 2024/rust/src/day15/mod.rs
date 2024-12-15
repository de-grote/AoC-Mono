use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, (grid, instructions)) = parse(input)?;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut position = None;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            match c {
                '#' => {
                    walls.insert(pos);
                }
                'O' => {
                    boxes.insert(pos);
                }
                '@' => position = Some(pos),
                _ => (),
            }
        }
    }
    let mut position = position.unwrap();
    for dir in instructions {
        let next = position + dir.unit_vector();
        if walls.contains(&next) {
            continue;
        }
        if boxes.contains(&next) {
            let mut nnext = next;
            while boxes.contains(&nnext) {
                nnext += dir.unit_vector();
            }
            if !walls.contains(&nnext) {
                boxes.remove(&next);
                boxes.insert(nnext);
                position = next;
            }
        } else {
            position = next;
        }
    }
    Ok(boxes
        .into_iter()
        .map(|v| 100 * v.y + v.x)
        .sum::<i32>()
        .to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, (grid, instructions)) = parse(input)?;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut position = None;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let pos = IVec2::new(2 * x as i32, y as i32);
            match c {
                '#' => {
                    walls.insert(pos);
                    walls.insert(pos + IVec2::X);
                }
                'O' => {
                    boxes.insert(pos);
                }
                '@' => position = Some(pos),
                _ => (),
            }
        }
    }
    let mut position = position.unwrap();
    for dir in instructions {
        let next = position + dir.unit_vector();
        if walls.contains(&next) {
            continue;
        }
        if let Some((removed, added)) = move_boxes(&walls, &boxes, next, dir, false) {
            for r in removed {
                boxes.remove(&r);
            }
            for a in added {
                boxes.insert(a);
            }

            position = next;
        }
    }
    Ok(boxes
        .into_iter()
        .map(|v| 100 * v.y + v.x)
        .sum::<i32>()
        .to_string())
}

fn move_boxes(
    walls: &HashSet<IVec2>,
    boxes: &HashSet<IVec2>,
    position: IVec2,
    direction: Direction,
    from_same: bool,
) -> Option<(HashSet<IVec2>, HashSet<IVec2>)> {
    if walls.contains(&position) {
        return None;
    }
    let lbox = boxes.contains(&position);
    let rbox = boxes.contains(&(position - IVec2::X));
    if lbox || rbox {
        let (mut removed, mut added) = move_boxes(
            walls,
            boxes,
            position + direction.unit_vector(),
            direction,
            false,
        )?;
        if matches!(direction, Direction::Up | Direction::Down) && !from_same {
            let same_box = if lbox {
                position + IVec2::X
            } else {
                position - IVec2::X
            };
            let (removed2, added2) = move_boxes(walls, boxes, same_box, direction, true)?;

            removed.extend(removed2);
            added.extend(added2);
        }
        if lbox {
            removed.insert(position);
            added.insert(position + direction.unit_vector());
        }
        Some((removed, added))
    } else {
        Some((HashSet::new(), HashSet::new()))
    }
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Direction>)> {
    separated_pair(
        separated_list1(line_ending, many1(satisfy(|c| !c.is_whitespace()))),
        multispace0,
        many1(alt((
            alt((
                value(Direction::Up, tag("^")),
                value(Direction::Down, tag("v")),
                value(Direction::Left, tag("<")),
                value(Direction::Right, tag(">")),
            ))
            .map(Some),
            value(None, multispace1),
        )))
        .map(|v| v.into_iter().flatten().collect_vec()),
    )(input)
}
