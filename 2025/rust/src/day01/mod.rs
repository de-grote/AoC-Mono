use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, _parsed) = parse(input)?;
    todo!()
}

pub fn part2(input: &str) -> Answer<usize> {
    let (_, _parsed) = parse(input)?;
    todo!()
}

fn parse(input: &str) -> IResult<&str, u32> {
    complete::u32.parse(input)
}
