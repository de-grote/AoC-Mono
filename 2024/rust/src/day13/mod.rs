use glam::DMat2;

use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, prizes) = parse(input)?;
    let res: i64 = prizes
        .into_iter()
        .map(|(a, b, prize)| {
            let mut ress = vec![];
            for ax in 0..=100 {
                let a2 = a * ax;
                if a2.cmpgt(prize).any() {
                    break;
                }
                let d = prize - a2;

                if d == I64Vec2::ZERO {
                    ress.push(I64Vec2::new(ax, 0));
                    continue;
                } else if d.x == 0 || d.y == 0 {
                    continue;
                }

                if d % b == I64Vec2::ZERO {
                    let bs = d / b;
                    if bs.x == bs.y {
                        ress.push(I64Vec2::new(ax, bs.x))
                    }
                }
            }
            ress.into_iter().map(|v| v.x * 3 + v.y).min().unwrap_or(0)
        })
        .sum();

    Ok(res.to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, prizes) = parse(input)?;
    let res: i64 = prizes
        .into_iter()
        .map(|(a, b, mut prize)| {
            let a_f = a.as_dvec2();
            let b_f = b.as_dvec2();
            prize += 10000000000000;
            let prize_f = prize.as_dvec2();
            let transform = DMat2::from_cols(a_f, b_f).inverse();
            let dists = (transform * prize_f).round();
            let sol = I64Vec2::new(dists.x as i64, dists.y as i64);
            if sol.x * a + sol.y * b == prize {
                3 * sol.x + sol.y
            } else {
                0
            }
        })
        .sum();

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(I64Vec2, I64Vec2, I64Vec2)>> {
    separated_list1(
        multispace1,
        tuple((
            delimited(
                tag("Button A: X+"),
                separated_pair(complete::i64, tag(", Y+"), complete::i64)
                    .map(|(x, y)| I64Vec2::new(x, y)),
                multispace0,
            ),
            delimited(
                tag("Button B: X+"),
                separated_pair(complete::i64, tag(", Y+"), complete::i64)
                    .map(|(x, y)| I64Vec2::new(x, y)),
                multispace0,
            ),
            preceded(
                tag("Prize: X="),
                separated_pair(complete::i64, tag(", Y="), complete::i64)
                    .map(|(x, y)| I64Vec2::new(x, y)),
            ),
        )),
    )(input)
}
