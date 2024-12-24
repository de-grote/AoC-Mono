#![allow(dead_code, unused_variables)]

use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, (mut values, connections)) = parse(input)?;

    let bools = connections
        .range("z00"..="z99")
        .map(|(&name, _)| get_value(&connections, &mut values, name))
        .collect_vec();

    let mut res: u64 = 0;
    for b in bools.into_iter().rev() {
        res <<= 1;
        res |= if b { 1 } else { 0 }
    }

    Ok(res.to_string())
}

fn get_value<'a>(
    connections: &BTreeMap<&'a str, (Operation, &'a str, &'a str)>,
    cache: &mut BTreeMap<&'a str, bool>,
    name: &'a str,
) -> bool {
    if let Some(&v) = cache.get(name) {
        return v;
    }
    let &(op, name1, name2) = connections.get(name).unwrap();
    let v1 = get_value(connections, cache, name1);
    let v2 = get_value(connections, cache, name2);
    let res = match op {
        Operation::And => v1 && v2,
        Operation::Or => v1 || v2,
        Operation::Xor => v1 ^ v2,
    };
    cache.insert(name, res);
    res
}

pub fn part2(input: &str) -> Answer {
    let (_, (_values, connections)) = parse(input)?;
    let max_register = connections.last_key_value().unwrap().0[1..]
        .parse::<u32>()
        .unwrap()
        .to_string();

    // let mut correct_connections = BTreeMap::new();
    // correct_connections.insert(Gates::Z(0), (Operation::Xor, Gates::X(0), Gates::Y(0)));
    // correct_connections.insert(Gates::Cout(0), (Operation::And, Gates::X(0), Gates::Y(0)));
    // for i in 1..max_register {
    //     correct_connections.insert(
    //         Gates::XorMain(i),
    //         (Operation::Xor, Gates::X(i), Gates::Y(i)),
    //     );
    //     correct_connections.insert(
    //         Gates::Z(i),
    //         (Operation::Xor, Gates::XorMain(i), Gates::Cout(i - 1)),
    //     );
    //     correct_connections.insert(
    //         Gates::AndMain(i),
    //         (Operation::And, Gates::X(i), Gates::Y(i)),
    //     );
    //     correct_connections.insert(
    //         Gates::AndCout(i),
    //         (Operation::And, Gates::XorMain(i), Gates::Cout(i - 1)),
    //     );
    //     correct_connections.insert(
    //         Gates::Cout(i),
    //         (Operation::Or, Gates::AndMain(i), Gates::AndCout(i)),
    //     );
    // }
    // correct_connections.remove(&Gates::Cout(max_register - 1));
    // correct_connections.insert(
    //     Gates::Z(max_register),
    //     (
    //         Operation::Or,
    //         Gates::AndMain(max_register - 1),
    //         Gates::AndCout(max_register - 1),
    //     ),
    // );

    // let mut wrongs = vec![];

    // let mut names = BTreeMap::new();
    // let mut check_later_and = Vec::new();
    // let mut check_later_or = Vec::new();
    // for (&name, &(op, n1, n2)) in connections.iter().rev() {
    //     // if let Some(r) = n1.strip_prefix('x') {
    //     //     names.insert(n1, Gates::X(r.parse::<u32>().unwrap()));
    //     // }
    //     // if let Some(r) = n2.strip_prefix('x') {
    //     //     names.insert(n2, Gates::X(r.parse::<u32>().unwrap()));
    //     // }
    //     // if let Some(r) = n1.strip_prefix('y') {
    //     //     names.insert(n1, Gates::Y(r.parse::<u32>().unwrap()));
    //     // }
    //     // if let Some(r) = n2.strip_prefix('y') {
    //     //     names.insert(n2, Gates::Y(r.parse::<u32>().unwrap()));
    //     // }
    //     if let Some(r) = name.strip_prefix('z') {
    //         names.insert(name, Gates::Z(r.parse::<u32>().unwrap()));
    //         continue;
    //     }
    //     match op {
    //         Operation::And => {
    //             if n1.starts_with('x') && n2.starts_with('y')
    //                 || n1.starts_with('y') && n2.starts_with('x')
    //             {
    //                 let x = n1[1..].parse::<u32>().unwrap();
    //                 if x != 0 {
    //                     names.insert(name, Gates::AndMain(n1[1..].parse::<u32>().unwrap()));
    //                 } else {
    //                     names.insert(name, Gates::Cout(0));
    //                 }
    //             } else if let Some(&x) = names.get(n1).or_else(|| names.get(n2)) {
    //                 match x {
    //                     Gates::Cout(x) => {
    //                         names.insert(name, Gates::AndCout(x + 1));
    //                     }
    //                     Gates::XorMain(x) => {
    //                         names.insert(name, Gates::AndCout(x));
    //                     }
    //                     _ => check_later_and.push(name),
    //                 }
    //             } else {
    //                 check_later_and.push(name);
    //             }
    //         }
    //         Operation::Or => {
    //             if let Some(&x) = names.get(n1).or_else(|| names.get(n2)) {
    //                 match x {
    //                     Gates::AndCout(x) | Gates::AndMain(x) => {
    //                         names.insert(name, Gates::Cout(x));
    //                     }
    //                     _ => check_later_or.push(name),
    //                 }
    //             } else {
    //                 check_later_or.push(name);
    //             }
    //         }
    //         Operation::Xor => {
    //             if let Ok(v) = n1[1..].parse::<u32>() {
    //                 names.insert(name, Gates::XorMain(v));
    //             } else {
    //                 wrongs.push(name);
    //             }
    //         }
    //     }
    // }
    // for name in check_later_and {
    //     let &(_op, n1, n2) = connections.get(name).unwrap();

    //     let &x = names.get(n1).or_else(|| names.get(n2)).unwrap();
    //     match x {
    //         Gates::Cout(x) => {
    //             names.insert(name, Gates::AndCout(x + 1));
    //         }
    //         Gates::XorMain(x) => {
    //             names.insert(name, Gates::AndCout(x));
    //         }
    //         _ => {
    //             wrongs.push(name);
    //         }
    //     }
    // }
    // for name in check_later_or {
    //     let &(_op, n1, n2) = connections.get(name).unwrap();
    //     if let Some(&(Gates::AndCout(x) | Gates::AndMain(x))) =
    //         names.get(n1).or_else(|| names.get(n2))
    //     {
    //         names.insert(name, Gates::Cout(x));
    //     } else {
    //         wrongs.push(name);
    //     }
    // }

    // dbg!(&wrongs);
    // dbg!(names.len(), wrongs.len(), connections.len());

    // let conn2 = connections
    //     .iter()
    //     .filter_map(|(&name, &(op, n1, n2))| {
    //         Some((
    //             names.get(name)?,
    //             (
    //                 name,
    //                 op,
    //                 names.get(n1).copied().or_else(|| {
    //                     let x = n1[1..].parse::<u32>().ok()?;
    //                     Some(if n1.starts_with('x') {
    //                         Gates::X(x)
    //                     } else {
    //                         Gates::Y(x)
    //                     })
    //                 }),
    //                 names.get(n2).copied().or_else(|| {
    //                     let x = n2[1..].parse::<u32>().ok()?;
    //                     Some(if n2.starts_with('x') {
    //                         Gates::X(x)
    //                     } else {
    //                         Gates::Y(x)
    //                     })
    //                 }),
    //             ),
    //         ))
    //     })
    //     .collect::<BTreeMap<_, _>>();

    // let wrong_gates = &conn2.keys().copied().copied().collect::<BTreeSet<_>>()
    //     ^ &correct_connections.keys().copied().collect::<BTreeSet<_>>();
    // dbg!(conn2.len());

    // let mut res = correct_connections
    //     .into_iter()
    //     .filter(|(gate, _)| !wrong_gates.contains(gate))
    //     .zip(conn2)
    //     .filter_map(|((_a, (op1, n11, n21)), (&_b, (name, op2, n12, n22)))| {
    //         let c1 = n12.map_or(true, |v| v == n11 || v == n21);
    //         let c2 = n22.map_or(true, |v| v == n21 || v == n11);
    //         // if name == "z36" {
    //         //     dbg!(op1, op2, n11, n21, n12, n22, c1, c2);
    //         // }
    //         (!(op1 == op2 && c1 && c2)).then_some(name)
    //     })
    //     .collect_vec();

    // wrongs.sort();
    // // res.extend(wrongs);
    // res.sort();

    // dbg!(res.len());

    let mut uses: BTreeMap<&str, u32> = BTreeMap::new();
    for (&k, &(_, n1, n2)) in connections.iter() {
        *uses.entry(k).or_default() += 1;
        *uses.entry(n1).or_default() += 1;
        *uses.entry(n2).or_default() += 1;
    }
    let res = connections
        .iter()
        .filter(|(&name, &(op, n1, n2))| {
            if name.starts_with('z') {
                if name.ends_with(&max_register) {
                    return op != Operation::Or;
                }
                return op != Operation::Xor;
            }
            let amt = if op == Operation::And { 2 } else { 3 };
            if uses.get(name) != Some(&amt) {
                return true;
            }
            // let op1 = connections.get(n1).map(|&(op, _, _)| op);
            // let op2 = connections.get(n2).map(|&(op, _, _)| op);
            // match op {
            //     Operation::And | Operation::Xor => {
            //         if !matches!(
            //             (op1, op2),
            //             (None, None)
            //                 | (Some(Operation::Xor), Some(Operation::Or))
            //                 | (Some(Operation::Or), Some(Operation::Xor))
            //         ) {
            //             return true;
            //         }
            //     }
            //     Operation::Or => {
            //         if !matches!((op1, op2), (Some(Operation::And), Some(Operation::And))) {
            //             return true;
            //         }
            //     }
            // }

            false
        })
        .collect_vec();

    dbg!(&res, res.len());

    Ok(res.into_iter().map(|(k, _)| k).sorted().join(","))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Gates {
    X(u32),
    Y(u32),
    Z(u32),
    XorMain(u32),
    AndMain(u32),
    AndCout(u32),
    Cout(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

fn parse(
    input: &str,
) -> IResult<
    &str,
    (
        BTreeMap<&str, bool>,
        BTreeMap<&str, (Operation, &str, &str)>,
    ),
> {
    separated_pair(
        fold_many1(
            terminated(
                separated_pair(
                    alphanumeric1,
                    tag(": "),
                    alt((value(true, tag("1")), value(false, tag("0")))),
                ),
                line_ending,
            ),
            BTreeMap::new,
            |mut acc, (name, value)| {
                acc.insert(name, value);
                acc
            },
        ),
        multispace0,
        fold_many1(
            tuple((
                terminated(alphanumeric1, space0),
                terminated(
                    alt((
                        value(Operation::And, tag("AND")),
                        value(Operation::Or, tag("OR")),
                        value(Operation::Xor, tag("XOR")),
                    )),
                    space0,
                ),
                terminated(alphanumeric1, tag(" -> ")),
                terminated(alphanumeric1, opt(line_ending)),
            )),
            BTreeMap::new,
            |mut acc, (name1, op, name2, store)| {
                acc.insert(store, (op, name1, name2));
                acc
            },
        ),
    )(input)
}
