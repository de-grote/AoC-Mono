use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u64> {
    let (_, (ranges, ids)) = parse(input)?;
    let mut ans = 0;

    for id in ids {
        for range in ranges.iter() {
            if id >= range.x && id <= range.y {
                ans += 1;
                break;
            }
        }
    }

    Ok(ans)
}

pub fn part2(input: &str) -> Answer<u64> {
    let (_, (mut ranges, _ids)) = parse(input)?;

    ranges.sort_by_key(|r| r.x);
    let mut new_ranges = vec![ranges[0]];
    for range in ranges.into_iter().skip(1) {
        let last = new_ranges.last_mut().unwrap();
        if range.x <= last.y {
            last.y = last.y.max(range.y);
        } else {
            new_ranges.push(range);
        }
    }

    Ok(new_ranges.into_iter().map(|v| v.y - v.x + 1).sum())
}

fn parse(input: &str) -> IResult<&str, (Vec<U64Vec2>, Vec<u64>)> {
    separated_pair(
        separated_list1(
            newline,
            separated_pair(complete::u64, tag("-"), complete::u64).map(|(x, y)| U64Vec2::new(x, y)),
        ),
        multispace1,
        separated_list0(newline, complete::u64),
    )
    .parse(input)
}
