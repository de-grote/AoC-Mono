use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, tests) = parse(input)?;

    let res: u64 = tests
        .into_iter()
        .filter(|(test, equation)| is_valid(*test, equation[0], &equation[1..]))
        .map(|(test, _)| test)
        .sum();

    Ok(res.to_string())
}

fn is_valid(test: u64, current: u64, rest: &[u64]) -> bool {
    if rest.is_empty() {
        return test == current;
    }
    is_valid(test, current + rest[0], &rest[1..]) || is_valid(test, current * rest[0], &rest[1..])
}

pub fn part2(input: &str) -> Answer {
    let (_, tests) = parse(input)?;

    let res: u64 = tests
        .into_iter()
        .filter(|(test, equation)| is_valid2(*test, equation[0], &equation[1..]))
        .map(|(test, _)| test)
        .sum();

    Ok(res.to_string())
}

fn is_valid2(test: u64, current: u64, rest: &[u64]) -> bool {
    if rest.is_empty() {
        return test == current;
    }
    is_valid2(test, current + rest[0], &rest[1..])
        || is_valid2(test, current * rest[0], &rest[1..])
        || {
            let st = rest[0].to_string().len() as u32;
            // let b = current.to_string() + &rest[0].to_string();
            is_valid2(test, current * 10u64.pow(st) + rest[0], &rest[1..])
        }
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}
