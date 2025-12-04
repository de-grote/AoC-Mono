use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u32> {
    let (_, map) = parse(input)?;
    let mut ans = 0;

    let (xlen, ylen) = map.size();
    for (x, y) in (0..xlen).cartesian_product(0..ylen) {
        if (map.at((x, y))) == Some(&'.') {
            continue;
        }
        let amt = adjacent_points(IVec2::new(x as i32, y as i32)).iter().filter(|pos| map.at(**pos) == Some(&'@')).count();
        if amt < 4 {
            ans += 1;
        }
    }

    Ok(ans)
}

pub fn part2(input: &str) -> Answer<u32> {
    let (_, mut map) = parse(input)?;
    let mut ans = 0;

    let (xlen, ylen) = map.size();
    let mut removed = 1;
    while removed != 0 {
        removed = 0;
        for (x, y) in (0..xlen).cartesian_product(0..ylen) {
            if map.at((x, y)) == Some(&'.') {
                continue;
            }
            let amt = adjacent_points(IVec2::new(x as i32, y as i32)).iter().filter(|pos| map.at(**pos) == Some(&'@')).count();
            if amt < 4 {
                removed += 1;
                ans += 1;
                *map.at_mut((x, y)).unwrap() = '.';
            }
        }
    }

    Ok(ans)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list0(newline, many1(satisfy(|c| !c.is_whitespace()))).parse(input)
}
