use crate::prelude::*;
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, constraint, microlp, variable};

pub fn part1(input: &str) -> Answer<usize> {
    let (_, machines) = parse(input)?;

    let mut res = 0;

    'outer: for (lights, schematics, _joltage) in machines {
        for i in 1..schematics.len() {
            for combi in schematics.iter().combinations(i) {
                let mut lights_sim = vec![false; lights.len()];
                for c in combi {
                    for idx in c {
                        lights_sim[*idx] ^= true;
                    }
                }
                if lights_sim == lights {
                    res += i;
                    continue 'outer;
                }
            }
        }
    }

    Ok(res)
}

pub fn part2(input: &str) -> Answer<usize> {
    let (_, machines) = parse(input)?;

    let mut res = 0;

    for (_lights, schematics, joltage) in machines {
        let mut vars = ProblemVariables::new();
        for _ in &schematics {
            vars.add(variable().min(0).integer());
        }
        let mut sum = Expression::default();
        let mut vars2 = Vec::new();
        for (v, _) in vars.iter_variables_with_def() {
            sum += v;
            vars2.push(v);
        }
        let sol = vars
            .minimise(&sum)
            .using(microlp)
            .with_all(joltage.into_iter().enumerate().map(|(i, jolt)| {
                let mut expr = Expression::default();
                for (j, schematic) in schematics.iter().enumerate() {
                    if schematic.contains(&i) {
                        expr += vars2[j];
                    }
                }
                constraint!(expr == jolt as u32)
            }))
            .solve()?;
        res += sol.eval(sum) as usize;
    }

    Ok(res)
}

fn parse(input: &str) -> IResult<&str, Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>> {
    separated_list1(
        multispace1,
        (
            delimited(
                tag("["),
                many0(satisfy(|c| c == '.' || c == '#').map(|c| c == '#')),
                tag("] "),
            ),
            separated_list1(
                space1,
                delimited(
                    tag("("),
                    separated_list1(tag(","), complete::usize),
                    tag(")"),
                ),
            ),
            delimited(
                tag(" {"),
                separated_list1(tag(","), complete::usize),
                tag("}"),
            ),
        ),
    )
    .parse(input)
}
