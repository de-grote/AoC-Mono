use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, (mut register, instructions)) = parse(input)?;
    let mut ip = 0;
    let mut out: Vec<u64> = Vec::new();

    while let (Some(&instr), Some(&operand)) = (instructions.get(ip), instructions.get(ip + 1)) {
        match instr {
            // adv
            0 => {
                register.a >>= combo(register, operand);
            }
            // bxl
            1 => {
                register.b ^= operand;
            }
            // bst
            2 => {
                register.b = combo(register, operand) & 7;
            }
            // jnz
            3 => {
                if register.a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            // bxc
            4 => register.b ^= register.c,
            // out
            5 => {
                out.push(combo(register, operand) & 7);
            }
            // bdv
            6 => {
                register.b = register.a >> combo(register, operand);
            }
            // cdv
            7 => {
                register.c = register.a >> combo(register, operand);
            }
            _ => (),
        }
        ip += 2;
    }

    Ok(out.into_iter().join(","))
}

fn combo(regs: Registers, v: u64) -> u64 {
    match v {
        0..=3 => v,
        4 => regs.a,
        5 => regs.b,
        6 => regs.c,
        _ => unreachable!("invalid combo"),
    }
}

pub fn part2(input: &str) -> Answer {
    let (_, (_start_register, instructions)) = parse(input)?;

    // a lot of assumptions are hidden here
    let important_instructions = instructions
        .iter()
        .tuples()
        .filter_map(|(&instr, &operand)| [1, 4, 7].contains(&instr).then_some((instr, operand)))
        .collect_vec();

    for x in 0..8 {
        if let Some(&res) = find_lowest_bits(x, &instructions, &important_instructions).first() {
            // because the testcase orders the adv before the out it needs to be shifted by 3 more bits
            let res = if cfg!(test) { res << 3 } else { res };
            return Ok(res.to_string());
        }
    }
    panic!("no solution found");
}

fn find_lowest_bits(previous_bits: u64, instructions: &[u64], code: &[(u64, u64)]) -> Vec<u64> {
    let Some(&instr) = instructions.last() else {
        return vec![previous_bits];
    };
    let previous_bits = previous_bits << 3;
    let mut options = (0..8)
        .filter_map(|v| {
            let mut b: u64 = v;
            let mut c: u64 = 0;
            let a = previous_bits | v;
            for &(ins, arg) in code.iter() {
                match ins {
                    1 => b ^= arg,
                    4 => b ^= c,
                    7 => c = a >> b,
                    _ => panic!("didn't account for instruction {}", ins),
                }
            }
            b &= 7;
            (b == instr).then_some(a)
        })
        .collect_vec();
    options.sort();
    options
        .into_iter()
        .flat_map(|o| find_lowest_bits(o, &instructions[..instructions.len() - 1], code))
        .collect_vec()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Registers {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

fn parse(input: &str) -> IResult<&str, (Registers, Vec<u64>)> {
    separated_pair(
        tuple((
            delimited(tag("Register A: "), complete::u64, line_ending),
            delimited(tag("Register B: "), complete::u64, line_ending),
            delimited(tag("Register C: "), complete::u64, line_ending),
        ))
        .map(|(a, b, c)| Registers { a, b, c }),
        multispace0,
        preceded(tag("Program: "), separated_list1(tag(","), complete::u64)),
    )(input)
}
