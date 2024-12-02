use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let res = grid
        .into_iter()
        .filter(|report| {
            report
                .iter()
                .tuple_windows()
                .all(|(a, b)| (1..=3).contains(&(a - b)))
                || report
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| (1..=3).contains(&(b - a)))
        })
        .count();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, grid) = parse(input)?;
    let res = grid
        .into_iter()
        .filter(|report| {
            report
                .iter()
                .tuple_windows()
                .all(|(a, b)| (1..=3).contains(&(a - b)))
                || report
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| (1..=3).contains(&(b - a)))
                || report.iter().combinations(report.len() - 1).any(|comb| {
                    comb.iter()
                        .tuple_windows()
                        .all(|(&a, &b)| (1..=3).contains(&(a - b)))
                        || comb
                            .iter()
                            .tuple_windows()
                            .all(|(&a, &b)| (1..=3).contains(&(b - a)))
                })
        })
        .count();

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}
