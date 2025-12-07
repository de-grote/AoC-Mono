use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u64> {
    let (_, grid) = parse(input)?;
    let mut beams = HashSet::new();
    for (idx, &c) in grid[0].iter().enumerate() {
        if c == 'S' {
            beams.insert(idx);
        }
    }
    let mut splits = 0;
    for row in grid.into_iter().skip(1) {
        let mut new_beams = beams.clone();
        for (idx, c) in row.into_iter().enumerate() {
            if c == '^' {
                if new_beams.remove(&idx) {
                    splits += 1;
                    new_beams.insert(idx + 1);
                    new_beams.insert(idx - 1);
                }
            }
        }
        beams = new_beams;
    }

    Ok(splits)
}

pub fn part2(input: &str) -> Answer<u64> {
    let (_, grid) = parse(input)?;
    let mut beams = HashMap::new();
    for (idx, &c) in grid[0].iter().enumerate() {
        if c == 'S' {
            beams.insert(idx, 1);
        }
    }
    for row in grid.into_iter().skip(1) {
        let mut new_beams = beams.clone();
        for (idx, c) in row.into_iter().enumerate() {
            if c == '^' {
                if let Some(val) = new_beams.remove(&idx) {
                    *new_beams.entry(idx + 1).or_default() += val;
                    *new_beams.entry(idx - 1).or_default() += val;
                }
            }
        }
        beams = new_beams;
    }

    Ok(beams.values().sum())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(multispace1, many1(satisfy(|c| !c.is_whitespace()))).parse(input)
}
