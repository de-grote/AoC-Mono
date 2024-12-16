use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;
    let grid: Vec<Vec<bool>> = grid
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = IVec2::new(x as i32, y as i32);
                        false
                    }
                    'E' => {
                        end = IVec2::new(x as i32, y as i32);
                        false
                    }
                    '#' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let mut heap = BinaryHeap::new();
    heap.push(PosData(0, start, Direction::Right));
    let mut been = HashSet::new();

    while let Some(PosData(score, position, direction)) = heap.pop() {
        if been.contains(&(position, direction)) {
            continue;
        }
        been.insert((position, direction));
        let next = position + direction.unit_vector();
        if next == end {
            return Ok((score + 1).to_string());
        }
        if grid.at(next) == Some(&false) {
            heap.push(PosData(score + 1, next, direction));
        }
        if grid.at(position + direction.rotate_left().unit_vector()) == Some(&false) {
            heap.push(PosData(
                score + 1000,
                position,
                direction.rotate_left(),
            ));
        }
        if grid.at(position + direction.rotate_right().unit_vector()) == Some(&false) {
            heap.push(PosData(
                score + 1000,
                position,
                direction.rotate_right(),
            ));
        }
    }

    panic!("no solution found");
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;
    let grid: Vec<Vec<bool>> = grid
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = IVec2::new(x as i32, y as i32);
                        false
                    }
                    'E' => {
                        end = IVec2::new(x as i32, y as i32);
                        false
                    }
                    '#' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let mut heap = BinaryHeap::new();
    heap.push(PosData(0, start, Direction::Right));
    let mut been: HashMap<(IVec2, Direction), u64> = HashMap::new();
    let mut end_dir = Direction::Up;

    while let Some(PosData(score, position, direction)) = heap.pop() {
        if been.contains_key(&(position, direction)) {
            continue;
        }
        been.insert((position, direction), score);
        if position == end {
            end_dir = direction;
            break;
        }
        let next = position + direction.unit_vector();
        if grid.at(next) == Some(&false) {
            heap.push(PosData(score + 1, next, direction));
        }
        if grid.at(position + direction.rotate_left().unit_vector()) == Some(&false) {
            heap.push(PosData(
                score + 1000,
                position,
                direction.rotate_left(),
            ));
        }
        if grid.at(position + direction.rotate_right().unit_vector()) == Some(&false) {
            heap.push(PosData(
                score + 1000,
                position,
                direction.rotate_right(),
            ));
        }
    }

    let mut to_check = HashSet::new();
    to_check.insert((end, end_dir));
    let mut all = HashSet::new();
    while let Some(&(pos, dir)) = to_check.iter().next() {
        to_check.remove(&(pos, dir));
        all.insert(pos);
        let score = been.get(&(pos, dir)).unwrap();
        if been.get(&(pos - dir.unit_vector(), dir)) == Some(&(score.wrapping_sub(1))) {
            to_check.insert((pos - dir.unit_vector(), dir));
        }
        if been.get(&(pos, dir.rotate_left())) == Some(&(score.wrapping_sub(1000))) {
            to_check.insert((pos, dir.rotate_left()));
        }
        if been.get(&(pos, dir.rotate_right())) == Some(&(score.wrapping_sub(1000))) {
            to_check.insert((pos, dir.rotate_right()));
        }
    }

    Ok(all.len().to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PosData(u64, IVec2, Direction);

impl Ord for PosData {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for PosData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(satisfy(|c| !c.is_whitespace())))(input)
}
