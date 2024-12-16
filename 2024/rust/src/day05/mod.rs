use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, (rules, updates)) = parse(input)?;

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (v, k) in rules {
        map.entry(k).or_default().push(v);
    }

    let res: u32 = updates
        .into_iter()
        .filter(|update| {
            update.iter().enumerate().all(|(i, v)| {
                if let Some(values) = map.get(v) {
                    !values.iter().any(|v| update[i..].contains(v))
                } else {
                    true
                }
            })
        })
        .map(|update| update[update.len() / 2])
        .sum();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, (rules, updates)) = parse(input)?;

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (v, k) in rules {
        map.entry(k).or_default().push(v);
    }

    let res: u32 = updates
        .into_iter()
        .filter(|update| {
            !update.iter().enumerate().all(|(i, v)| {
                if let Some(values) = map.get(v) {
                    !values.iter().any(|v| update[i..].contains(v))
                } else {
                    true
                }
            })
        })
        .map(|mut update| {
            update.sort_by(|x, y| match (map.get(x), map.get(y)) {
                (Some(f), _) if f.contains(y) => Ordering::Less,
                (_, Some(f)) if f.contains(y) => Ordering::Greater,
                _ => Ordering::Equal,
            });
            update[update.len() / 2]
        })
        .sum();

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (input, rules) = separated_list1(
        line_ending,
        separated_pair(complete::u32, tag("|"), complete::u32),
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, update) =
        separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)?;
    Ok((input, (rules, update)))
}
