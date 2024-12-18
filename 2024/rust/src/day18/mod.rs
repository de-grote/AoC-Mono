use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, coords) = parse(input)?;

    let limit = if cfg!(test) {12} else {1024};

    let grid = coords.into_iter().take(limit).collect::<HashSet<_>>();

    let grid_size = if cfg!(test) {
        IVec2::splat(6)
    } else {
        IVec2::splat(70)
    };

    let mut going = HashSet::new();
    let mut been = HashSet::new();
    going.insert(IVec2::ZERO);
    for i in 1.. {
        let mut next = HashSet::new();
        while let Some(&pos) = going.iter().next() {
            going.remove(&pos);
            for p in orthogonal_points(pos) {
                if p == grid_size {
                    return Ok(i.to_string())
                }
                if !grid.contains(&p) && !been.contains(&p) && p.clamp(IVec2::ZERO, grid_size) == p {
                    next.insert(p);
                    been.insert(p);
                }
            }
        }
        going.extend(next);
    }

    panic!("no solution found")
}

pub fn part2(input: &str) -> Answer {
    let (_, coords) = parse(input)?;

    let limit = if cfg!(test) {12} else {1024};

    let mut grid = coords.iter().take(limit).copied().collect::<HashSet<_>>();

    let grid_size = if cfg!(test) {
        IVec2::splat(6)
    } else {
        IVec2::splat(70)
    };

    'outer: for &i in coords.iter().skip(limit) {
        grid.insert(i);
        
        let mut going = HashSet::new();
        let mut been = HashSet::new();
        going.insert(IVec2::ZERO);
        while !going.is_empty() {
            let mut next = HashSet::new();
            while let Some(&pos) = going.iter().next() {
                going.remove(&pos);
                for p in orthogonal_points(pos) {
                    if p == grid_size {
                        continue 'outer;
                    }
                    if !grid.contains(&p) && !been.contains(&p) && p.clamp(IVec2::ZERO, grid_size) == p {
                        next.insert(p);
                        been.insert(p);
                    }
                }
            }
            going.extend(next);
        }
        return Ok(format!("{},{}", i.x, i.y))
    }

    panic!("no solution found")
}

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    separated_list1(
        line_ending,
        separated_pair(complete::i32, tag(","), complete::i32).map(|(x, y)| IVec2::new(x, y)),
    )(input)
}
