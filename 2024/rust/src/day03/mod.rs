use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, pairs) = parse(input)?;
    let res: u32 = pairs.into_iter().map(|(x, y)| x * y).sum();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, pairs) = parse2(input)?;
    let mut res: u32 = 0;
    let mut enabled = true;
    for d in pairs {
        match d {
            DoOrMul::Do => enabled = true,
            DoOrMul::Dont => enabled = false,
            DoOrMul::Mul(x, y) => {
                if enabled {
                    res += x * y;
                }
            }
        }
    }

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, mut r) = alt((
        eof.map(|_| vec![]),
        valid_mul.map(|x| vec![x]),
        preceded(take_till1(|c| c == 'm'), parse),
        preceded(take(1usize), parse),
    ))(input)?;
    let (input, rest) = alt((eof.map(|_| vec![]), parse))(input)?;
    r.extend(rest);
    Ok((input, r))
}

fn valid_mul(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(digits, tag(","), digits),
        tag(")"),
    )(input)
}

fn digits(input: &str) -> IResult<&str, u32> {
    let (input, x) = complete::u32(input)?;
    if x >= 1000 {
        return fail(input);
    }
    Ok((input, x))
}

#[derive(Debug, Clone, Copy)]
enum DoOrMul {
    Do,
    Dont,
    Mul(u32, u32),
}

fn parse2(input: &str) -> IResult<&str, Vec<DoOrMul>> {
    let (input, mut r) = alt((
        eof.map(|_| vec![]),
        tag("do()").map(|_| vec![DoOrMul::Do]),
        tag("don't()").map(|_| vec![DoOrMul::Dont]),
        valid_mul.map(|(x, y)| vec![DoOrMul::Mul(x, y)]),
        preceded(take_till1(|c| c == 'm' || c == 'd'), parse2),
        preceded(take(1usize), parse2),
    ))(input)?;
    let (input, rest) = alt((eof.map(|_| vec![]), parse2))(input)?;
    r.extend(rest);
    Ok((input, r))
}
