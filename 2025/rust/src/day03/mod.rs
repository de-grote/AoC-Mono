use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u32> {
    let (_, banks) = parse(input)?;
    let mut ans = 0;

    for bank in banks {
        let (idx, &max) = bank.iter().enumerate().rev().skip(1).max_by_key(|(_i, v)| *v).unwrap();
        let &max2 = bank.iter().skip(idx + 1).max().unwrap();
        ans += max * 10 + max2;
    }

    Ok(ans)
}

pub fn part2(input: &str) -> Answer<u64> {
    let (_, banks) = parse(input)?;
    let mut ans: u64 = 0;

    for bank in banks {
        let mut idx = 0;
        for i in (0..12).rev() {
            let (new_idx, &max) = bank.iter().enumerate().skip(idx).rev().skip(i).max_by_key(|(_i, v)| *v).unwrap();
            ans += max as u64 * 10u64.pow(i as u32);
            idx = new_idx + 1;
        }
    }

    Ok(ans)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list0(newline, many1(anychar.map_opt(|c| c.to_digit(10)))).parse(input)
}
