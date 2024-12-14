use std::{cmp::Ordering, io::stdin};

use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, robots) = parse(input)?;
    let size = if cfg!(test) {
        IVec2::new(11, 7)
    } else {
        IVec2::new(101, 103)
    };
    let half_size = size / 2;
    let mut corners = [0; 4];
    for (pos, vel) in robots {
        let final_pos = (pos + vel * 100).rem_euclid(size);
        match (final_pos.x.cmp(&half_size.x), final_pos.y.cmp(&half_size.y)) {
            (Ordering::Less, Ordering::Less) => corners[0] += 1,
            (Ordering::Less, Ordering::Greater) => corners[1] += 1,
            (Ordering::Greater, Ordering::Less) => corners[2] += 1,
            (Ordering::Greater, Ordering::Greater) => corners[3] += 1,
            _ => (),
        }
    }
    Ok(corners.into_iter().product::<i32>().to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, robots) = parse(input)?;
    let size = IVec2::new(101, 103);
    for i in 1.. {
        let poss: HashSet<IVec2> = robots
            .iter()
            .map(|(pos, vel)| (pos + vel * i).rem_euclid(size))
            .collect();
        if poss.len() == robots.len() {
            print_robots(&poss);
            println!("{i}");
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            if s.starts_with('y') {
                return Ok(i.to_string());
            }
        }
    }
    unreachable!()
}

fn print_robots(poss: &HashSet<IVec2>) {
    for y in 0..103 {
        for x in 0..101 {
            let pos = IVec2::new(x, y);
            let c = if poss.contains(&pos) { '#' } else { '.' };
            print!("{}", c);
        }
        println!()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(IVec2, IVec2)>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(
                tag("p="),
                separated_pair(complete::i32, tag(","), complete::i32)
                    .map(|(x, y)| IVec2::new(x, y)),
            ),
            space1,
            preceded(
                tag("v="),
                separated_pair(complete::i32, tag(","), complete::i32)
                    .map(|(x, y)| IVec2::new(x, y)),
            ),
        ),
    )(input)
}
