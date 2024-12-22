use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, numbers) = parse(input)?;

    let res = numbers
        .into_iter()
        .map(|mut secret_number| {
            for _ in 0..2000 {
                secret_number ^= secret_number << 6;
                secret_number &= 16777215;
                secret_number ^= secret_number >> 5;
                secret_number ^= secret_number << 11;
                secret_number &= 16777215;
            }
            secret_number as u64
        })
        .sum::<u64>();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, numbers) = parse(input)?;

    let mut best = 0;

    let prices = numbers
        .iter()
        .map(|secret_number| {
            let mut secret_number = *secret_number;
            (0..2000)
                .map(|_| {
                    secret_number ^= secret_number << 6;
                    secret_number &= 16777215;
                    secret_number ^= secret_number >> 5;
                    secret_number ^= secret_number << 11;
                    secret_number &= 16777215;
                    secret_number as i32 % 10
                })
                .collect_vec()
        })
        .collect_vec();

    for range in iproduct!((-3..=3), (-3..=3), (-3..=3), (-3..=3)) {
        let bananas = prices
            .iter()
            .map(|price| {
                price
                    .iter()
                    .copied()
                    .tuple_windows::<(_, _, _, _, _)>()
                    .find_map(|(a, b, c, d, e)| {
                        ((a - b, b - c, c - d, d - e) == range).then_some(e)
                    })
                    .unwrap_or_default() as i64
            })
            .sum::<i64>();
        best = best.max(bananas);
    }

    Ok(best.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(line_ending, complete::u32)(input)
}
