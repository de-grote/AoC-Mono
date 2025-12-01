use crate::prelude::*;

pub fn part1(input: &str) -> Answer<i32> {
    let (_, dials) = parse(input)?;
    let mut dial = 50;
    let mut ans = 0;
    for (dir, amount) in dials {
        if dir == 'L' {
            dial -= amount;
        } else {
            dial += amount;
        }
        dial %= 100;
        if dial == 0 {
            ans += 1;
        }
    }
    Ok(ans)
}

pub fn part2(input: &str) -> Answer<i32> {
    let (_, dials) = parse(input)?;
    let mut dial = 50;
    let mut ans = 0;
    for (dir, amount) in dials {
        ans += amount / 100;
        let prev = dial;
        if dir == 'L' {
            dial -= amount % 100;
        } else {
            dial += amount % 100;
        }
        let old = dial;
        dial = (dial + 100) % 100;
        if (old != dial || dial == 0) && prev != 0 {
            ans += 1;
        }
    }
    Ok(ans)
}

fn parse(input: &str) -> IResult<&str, Vec<(char, i32)>> {
    separated_list0(line_ending, (satisfy(|c| c == 'L' || c == 'R'), complete::i32)).parse(input)
}
