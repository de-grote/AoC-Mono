use good_lp::{Expression, Solution, SolverModel, constraint, microlp, variable, variables};

use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u32> {
    let (_, (shapes, regions)) = parse(input)?;

    let mut res = 0;

    let mut unique_shapes = Vec::new();
    for shape in shapes.iter() {
        let mut empty = Vec::with_capacity(3);
        empty.push(Vec::from_iter([false; 3]));
        empty.push(Vec::from_iter([false; 3]));
        empty.push(Vec::from_iter([false; 3]));
        let mut s = [
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ];

        for x in 0..=2 {
            for y in 0..=2 {
                s[0][x][y] = shape[x][y];
                s[1][x][y] = shape[y][x];
                s[2][x][y] = shape[2 - x][y];
                s[3][x][y] = shape[2 - y][x];
                s[4][x][y] = shape[x][2 - y];
                s[5][x][y] = shape[y][2 - x];
                s[6][x][y] = shape[2 - x][2 - y];
                s[7][x][y] = shape[2 - y][2 - x];
            }
        }
        unique_shapes.push(s.into_iter().unique().collect_vec());
    }

    let sizes = shapes
        .iter()
        .map(|x| x.iter().flatten().filter(|x| **x).count())
        .collect_vec();

    for (size, presents) in regions {
        if presents
            .iter()
            .zip(sizes.iter())
            .map(|(a, b)| a * *b as u32)
            .sum::<u32>()
            > size.element_product()
        {
            continue;
        }
        if presents.iter().sum::<u32>() * 9 <= size.element_product() {
            res += 1;
            continue;
        }
        // all this stuff below here is unnecessary, but does work it just takes >5h to finish (I stopped it after that)
        variables! {
            vars:
                grid[size.element_product() as usize] (binary);
        }
        let mut expr_grid = vec![Expression::default(); size.element_product() as usize];
        let mut constraints = Vec::new();
        let index = |x: u32, y: u32| (x * (size.y) + y) as usize;
        let index2 = |x: u32, y: u32| (x * (size.y - 2) + y) as usize;
        for (unique_shapes, present) in unique_shapes.iter().zip(presents) {
            let mut sum = Expression::default();
            for shape in unique_shapes {
                let v = vars.add_vector(variable().binary(), (size - 2).element_product() as usize);
                for x in 0..size.x - 2 {
                    for y in 0..size.y - 2 {
                        for dx in 0..=2 {
                            for dy in 0..=2 {
                                if shape[dx as usize][dy as usize] {
                                    expr_grid[index(x + dx, y + dy)] += v[index2(x, y)];
                                }
                            }
                        }
                    }
                }
                sum = v.iter().fold(sum, |acc, var| acc + var);
            }

            constraints.push(constraint!(sum == present));
        }
        for (expr, var) in expr_grid.into_iter().zip(grid.iter()) {
            constraints.push(constraint!(expr <= var));
        }
        // microlp is very fast at small problems but doesn't scale well to the 50-100k variables I create
        let solver = microlp;
        #[cfg(feature = "day12_ilp")]
        let solver = {
            use good_lp::highs;
            highs
        };
        let sol = vars
            .minimise(grid.iter().fold(Expression::default(), |acc, g| acc + g))
            // .minimise(Expression::default())
            .using(solver)
            .with_all(constraints)
            .solve();
        if let Err(e) = &sol {
            dbg!(e);
        }
        if let Ok(s) = sol {
            res += 1;
            for y in 0..size.y {
                for x in 0..size.x {
                    print!(
                        "{}",
                        if s.eval(grid[index(x, y)]) > 0.5 {
                            '#'
                        } else {
                            '.'
                        }
                    )
                }
                println!();
            }
        }
    }

    Ok(res)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<Vec<bool>>>, Vec<(UVec2, Vec<u32>)>)> {
    separated_pair(
        preceded(
            (complete::u32, tag(":"), line_ending),
            separated_list1(
                (multispace0, complete::u32, tag(":"), line_ending),
                separated_list1(
                    (line_ending, not(line_ending)),
                    many0(satisfy(|c| c == '.' || c == '#').map(|c| c == '#')),
                ),
            ),
        ),
        multispace0,
        separated_list1(
            line_ending,
            separated_pair(
                separated_pair(complete::u32, tag("x"), complete::u32)
                    .map(|(x, y)| UVec2::new(x, y)),
                tag(": "),
                separated_list1(space1, complete::u32),
            ),
        ),
    )
    .parse(input)
}
