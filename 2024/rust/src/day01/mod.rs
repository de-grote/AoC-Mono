use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, _var) = parse(input)?;

    Ok(input.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, _var) = parse(input)?;

    Ok(input.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, space0)(input)
}
