use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u64> {
    let (_, ranges) = parse(input)?;
    let mut ans = 0;
    for range in ranges {
        for r in range.x..=range.y {
            let s = r.to_string();
            if s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..] {
                ans += r;
            }
        }
    }
    Ok(ans)
}

pub fn part2(input: &str) -> Answer<u64> {
    let (_, ranges) = parse(input)?;
    let mut ans = 0;
    for range in ranges {
        for r in range.x..=range.y {
            let s = r.to_string();
            for i in 1..s.len() {
                if s.len() % i == 0 {
                    if s[..i].to_string().repeat(s.len() / i) == s {
                        ans += r;
                        break;
                    }
                }
            }
        }
    }
    Ok(ans)
}

fn parse(input: &str) -> IResult<&str, Vec<U64Vec2>> {
    separated_list0(
        tag(","),
        separated_pair(complete::u64, tag("-"), complete::u64).map(|(x, y)| U64Vec2::new(x, y)),
    )
    .parse(input)
}
