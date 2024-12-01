use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, pairs) = parse(input)?;
    let (mut a, mut b): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    a.sort();
    b.sort();
    let res: u32 = a.into_iter().zip(b).map(|(f, s)| u32::abs_diff(f, s)).sum();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, pairs) = parse(input)?;
    let (a, b): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    let a1 = a.into_iter().counts();
    let b1 = b.into_iter().counts();
    let mut res = 0;
    for (k, v) in a1.into_iter() {
        res += k as usize * v * b1.get(&k).unwrap_or(&0);
    }
    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        line_ending,
        separated_pair(complete::u32, space1, complete::u32),
    )(input)
}
