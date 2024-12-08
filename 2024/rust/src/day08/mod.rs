use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let (width, height) = grid.size();
    let mut map: HashMap<char, Vec<IVec2>> = HashMap::new();
    for (y, row) in grid.into_iter().enumerate() {
        for (x, v) in row.into_iter().enumerate() {
            match v {
                '.' => {}
                a => {
                    map.entry(a)
                        .or_default()
                        .push(IVec2::new(x as i32, y as i32));
                }
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, positions) in map.into_iter() {
        positions.iter().combinations(2).for_each(|pair| {
            let diff = pair[0] - pair[1];
            let a = pair[0] + diff;
            let b = pair[1] - diff;
            for c in [a, b] {
                if c.clamp(IVec2::ZERO, IVec2::new(width as i32 - 1, height as i32 - 1)) == c {
                    antinodes.insert(c);
                }
            }
        });
    }

    Ok(antinodes.len().to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let (width, height) = grid.size();
    let mut map: HashMap<char, Vec<IVec2>> = HashMap::new();
    for (y, row) in grid.into_iter().enumerate() {
        for (x, v) in row.into_iter().enumerate() {
            match v {
                '.' => {}
                a => {
                    map.entry(a)
                        .or_default()
                        .push(IVec2::new(x as i32, y as i32));
                }
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, positions) in map.into_iter() {
        positions.iter().combinations(2).for_each(|pair| {
            let diff = pair[0] - pair[1];
            let mut a = *pair[0];
            let mut b = *pair[1];

            while a.clamp(IVec2::ZERO, IVec2::new(width as i32 - 1, height as i32 - 1)) == a {
                antinodes.insert(a);
                a += diff;
            }
            while b.clamp(IVec2::ZERO, IVec2::new(width as i32 - 1, height as i32 - 1)) == b {
                antinodes.insert(b);
                b -= diff;
            }
        });
    }

    Ok(antinodes.len().to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    // separated_list1(line_ending, many1(many_m_n(1, 1, anychar).map(|v| v[0])))(input)
    Ok(("", input.lines().map(|l| l.chars().collect()).collect()))
}
