use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, r) = parse(input)?;
    let mut rocks: HashMap<u64, u64> = HashMap::new();
    for rock in r {
        *rocks.entry(rock).or_default() += 1;
    }

    for _ in 0..25 {
        let mut new = HashMap::new();
        for (rock, amount) in rocks {
            if rock == 0 {
                *new.entry(1).or_default() += amount;
            } else {
                let digits = rock.ilog10() + 1;
                if digits % 2 == 0 {
                    let v = 10u64.pow(digits / 2);
                    *new.entry(rock / v).or_default() += amount;
                    *new.entry(rock % v).or_default() += amount;
                } else {
                    *new.entry(rock * 2024).or_default() += amount;
                }
            }
        }
        rocks = new;
    }

    Ok(rocks.values().sum::<u64>().to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, r) = parse(input)?;
    let mut rocks: HashMap<u64, u64> = HashMap::new();
    for rock in r {
        *rocks.entry(rock).or_default() += 1;
    }

    for _ in 0..75 {
        let mut new = HashMap::new();
        for (rock, amount) in rocks {
            if rock == 0 {
                *new.entry(1).or_default() += amount;
            } else {
                let digits = rock.ilog10() + 1;
                if digits % 2 == 0 {
                    let v = 10u64.pow(digits / 2);
                    *new.entry(rock / v).or_default() += amount;
                    *new.entry(rock % v).or_default() += amount;
                } else {
                    *new.entry(rock * 2024).or_default() += amount;
                }
            }
        }
        rocks = new;
    }

    Ok(rocks.values().sum::<u64>().to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}
