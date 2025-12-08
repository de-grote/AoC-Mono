use glam::I64Vec3;

use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u64> {
    let (_, coords) = parse(input)?;

    let mut pairs = coords
        .iter()
        .copied()
        .combinations(2)
        .map(|c| (c[0], c[1]))
        .collect::<HashSet<_>>();

    let max_connections = if cfg!(test) { 10 } else { 1000 };
    let mut groups: HashMap<I64Vec3, u64> = HashMap::new();
    let mut new_idx = 0;

    for _ in 0..max_connections {
        let closest_pair = *pairs
            .iter()
            .min_by_key(|(x, y)| x.distance_squared(*y))
            .unwrap();
        // dbg!(closest_pair);
        pairs.remove(&closest_pair);
        let idx1 = groups.get(&closest_pair.0);
        let idx2 = groups.get(&closest_pair.1);
        match (idx1, idx2) {
            (None, None) => {
                groups.insert(closest_pair.0, new_idx);
                groups.insert(closest_pair.1, new_idx);
                new_idx += 1;
            }
            (None, Some(idx)) => {
                groups.insert(closest_pair.0, *idx);
            }
            (Some(idx), None) => {
                groups.insert(closest_pair.1, *idx);
            }
            (Some(&idx1), Some(&idx2)) => {
                if idx1 != idx2 {
                    for (_, v) in groups.iter_mut().filter(|(_, v)| v == &&idx1) {
                        *v = idx2;
                    }
                }
            }
        }
    }
    // dbg!(&groups);
    let mut group_counts: HashMap<u64, u64> = HashMap::new();
    for (_, v) in groups {
        *group_counts.entry(v).or_default() += 1;
    }
    let (&k, &max) = group_counts.iter().max_by_key(|(_, v)| **v).unwrap();
    group_counts.remove(&k);
    let (&k, &max2) = group_counts.iter().max_by_key(|(_, v)| **v).unwrap();
    group_counts.remove(&k);
    let max3 = *group_counts.values().max().unwrap_or(&1);
    Ok(max * max2 * max3)
}

pub fn part2(input: &str) -> Answer<i64> {
    let (_, coords) = parse(input)?;

    let mut pairs = coords
        .iter()
        .copied()
        .combinations(2)
        .map(|c| (c[0], c[1]))
        .collect::<HashSet<_>>();

    let mut groups: HashMap<I64Vec3, u64> = HashMap::new();
    let mut new_idx = 0;
    let mut group_count = coords.len();

    for _ in 0.. {
        let closest_pair = *pairs
            .iter()
            .min_by_key(|(x, y)| x.distance_squared(*y))
            .unwrap();
        pairs.remove(&closest_pair);
        let idx1 = groups.get(&closest_pair.0);
        let idx2 = groups.get(&closest_pair.1);
        match (idx1, idx2) {
            (None, None) => {
                groups.insert(closest_pair.0, new_idx);
                groups.insert(closest_pair.1, new_idx);
                new_idx += 1;
                group_count -= 1;
            }
            (None, Some(idx)) => {
                groups.insert(closest_pair.0, *idx);
                group_count -= 1;
            }
            (Some(idx), None) => {
                groups.insert(closest_pair.1, *idx);
                group_count -= 1;
            }
            (Some(&idx1), Some(&idx2)) => {
                if idx1 != idx2 {
                    for (_, v) in groups.iter_mut().filter(|(_, v)| v == &&idx1) {
                        *v = idx2;
                    }
                    group_count -= 1;
                }
            }
        }
        if group_count == 1 {
            return Ok(closest_pair.0.x * closest_pair.1.x)
        }
    }
    Ok(0)

}

fn parse(input: &str) -> IResult<&str, Vec<I64Vec3>> {
    separated_list1(
        multispace1,
        (
            terminated(complete::i64, tag(",")),
            terminated(complete::i64, tag(",")),
            complete::i64,
        )
            .map(|(x, y, z)| I64Vec3::new(x, y, z)),
    )
    .parse(input)
}
