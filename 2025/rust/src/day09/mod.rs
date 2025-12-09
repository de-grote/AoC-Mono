use crate::prelude::*;

pub fn part1(input: &str) -> Answer<i64> {
    let (_, coords) = parse(input)?;

    let res = coords
        .into_iter()
        .combinations(2)
        .map(|v| ((v[0] - v[1]).abs() + I64Vec2::new(1, 1)).element_product())
        .max()
        .unwrap();

    Ok(res)
}

pub fn part2(input: &str) -> Answer<i64> {
    let (_, coords) = parse(input)?;

    let mut xs = BTreeMap::new();
    let mut ys = BTreeMap::new();
    for (coord1, coord2) in coords.iter().circular_tuple_windows() {
        if coord1.x == coord2.x {
            xs.insert(coord1.x, I64Vec2::new(coord1.y, coord2.y));
        } else {
            ys.insert(coord1.y, I64Vec2::new(coord1.x, coord2.x));
        }
    }

    let mut maximal = 0;

    for v in coords.into_iter().combinations(2) {
        let (f, s) = (v[0], v[1]);
        let (minv, maxv) = (f.min(s), f.max(s));
        let x_range = xs
            .range(minv.x..=maxv.x)
            .filter(|(_, v)| (v.x.max(v.y) > minv.y) && (v.x.min(v.y) < maxv.y))
            .filter(|(x, v)| {
                if v.x < v.y {
                    **x < minv.x
                } else {
                    **x > maxv.x
                }
            })
            .count();
        let y_range = ys
            .range(minv.y..=maxv.y)
            .filter(|(_, v)| (v.x.max(v.y) > minv.x) && (v.x.min(v.y) < maxv.x))
            .filter(|(y, v)| {
                if v.x < v.y {
                    **y > minv.y
                } else {
                    **y < maxv.y
                }
            })
            .count();
        if x_range == 0 && y_range == 0 {
            maximal = ((f - s).abs() + I64Vec2::new(1, 1))
                .element_product()
                .max(maximal);
        }
    }

    Ok(maximal)
}

fn parse(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(
        multispace1,
        separated_pair(complete::i64, tag(","), complete::i64).map(|(x, y)| I64Vec2::new(x, y)),
    )
    .parse(input)
}
