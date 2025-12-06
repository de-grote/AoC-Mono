use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u64> {
    let (_, (numbers, ops)) = parse(input)?;
    let mut ans = 0;

    for (c, op) in numbers.columns().zip(ops) {
        ans += match op {
            '+' => c.sum::<u64>(),
            _ => c.product::<u64>(),
        };
    }

    Ok(ans)
}

pub fn part2(input: &str) -> Answer<u64> {
    let input = input
        .to_string()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
        .transpose()
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .collect_vec();
    let mut ans = 0;
    let mut op = ' ';
    let mut buf = Vec::new();
    for mut line in input.into_iter().chain([" ".to_string()]) {
        let t = line.pop().unwrap();
        if t != ' ' {
            op = t;
        }
        match line.trim().parse::<u64>() {
            Ok(v) => buf.push(v),
            Err(_) => {
                ans += match op {
                    '+' => buf.into_iter().sum::<u64>(),
                    _ => buf.into_iter().product::<u64>(),
                };
                buf = Vec::new()
            }
        }
    }
    Ok(ans)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<char>)> {
    separated_pair(
        separated_list1(multispace1, separated_list1(space1, complete::u64)),
        multispace1,
        separated_list0(space1, satisfy(|c| c == '*' || c == '+')),
    )
    .parse(input)
}
