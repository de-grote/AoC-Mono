#![allow(dead_code)]

pub mod part1 {
    enum Chips {
        Zero,
        One(u8),
        Two(u8, u8)
    }
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut bots: HashMap<u8, Chips> = HashMap::new();
        let mut commands: Vec<(u8, u8, u8, bool, bool)> = Vec::new();
        for i in input.split("\n").map(str::trim_end) {
            if i.starts_with('v') {
                let value = i.split_whitespace().nth(1)?.parse().ok()?;
                let bot = i.split_whitespace().last()?.parse().ok()?;
                if let Some(Chips::One(o)) = bots.insert(bot, Chips::One(value)) {
                    bots.insert(bot, Chips::Two(value, o));
                }
            } else {
                let bot = i.split_whitespace().nth(1)?.parse().ok()?;
                let low = i.split_whitespace().nth(6)?.parse().ok()?;
                let high = i.split_whitespace().last()?.parse().ok()?;
                let low2bot = i.split_whitespace().nth(5) == Some("bot");
                let high2bot = i.split_whitespace().nth(10) == Some("bot");
                commands.push((bot, low, high, low2bot, high2bot));
                if !bots.contains_key(&bot) {
                    bots.insert(bot, Chips::Zero);
                }
            }
        }
        let mut index = 0;
        while !commands.is_empty() {
            index = (index + 1) % commands.len();
            let (bot, low, high, low2bot, high2bot) = commands[index];
            if let Some(Chips::Two(a, b)) = bots.get(&bot) {
                let max;
                let min;
                if a > b {
                    max = *a;
                    min = *b;
                } else {
                    max = *b;
                    min = *a;
                }
                if max == 61 && min == 17 {
                    println!("{}", bot);
                    return Some(());
                }
                if low2bot {
                    bots.entry(low).and_modify(|x| {
                        match x {
                            Chips::Zero => {*x = Chips::One(min)},
                            Chips::One(y) => {*x = Chips::Two(*y, min)},
                            _ => (),
                        }
                    });
                }
                if high2bot {
                    bots.entry(high).and_modify(|x| {
                        match x {
                            Chips::Zero => {*x = Chips::One(max)},
                            Chips::One(y) => {*x = Chips::Two(*y, max)},
                            _ => (),
                        }
                    });
                }
                commands.remove(index);
            };
        }
        None
    }
}


pub mod part2 {
    enum Chips {
        Zero,
        One(u8),
        Two(u8, u8)
    }
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut bots: HashMap<u8, Chips> = HashMap::new();
        let mut commands: Vec<(u8, u8, u8, bool, bool)> = Vec::new();
        let mut out: [u32; 3] = Default::default();
        for i in input.split("\n").map(str::trim_end) {
            if i.starts_with('v') {
                let value = i.split_whitespace().nth(1)?.parse().ok()?;
                let bot = i.split_whitespace().last()?.parse().ok()?;
                if let Some(Chips::One(o)) = bots.insert(bot, Chips::One(value)) {
                    bots.insert(bot, Chips::Two(value, o));
                }
            } else {
                let bot = i.split_whitespace().nth(1)?.parse().ok()?;
                let low = i.split_whitespace().nth(6)?.parse().ok()?;
                let high = i.split_whitespace().last()?.parse().ok()?;
                let low2bot = i.split_whitespace().nth(5) == Some("bot");
                let high2bot = i.split_whitespace().nth(10) == Some("bot");
                commands.push((bot, low, high, low2bot, high2bot));
                if !bots.contains_key(&bot) {
                    bots.insert(bot, Chips::Zero);
                }
            }
        }
        let mut index = 0;
        while !commands.is_empty() {
            index = (index + 1) % commands.len();
            let (bot, low, high, low2bot, high2bot) = commands[index];
            if let Some(Chips::Two(a, b)) = bots.get(&bot) {
                let max;
                let min;
                if a > b {
                    max = *a;
                    min = *b;
                } else {
                    max = *b;
                    min = *a;
                }
                if low2bot {
                    bots.entry(low).and_modify(|x| {
                        match x {
                            Chips::Zero => {*x = Chips::One(min)},
                            Chips::One(y) => {*x = Chips::Two(*y, min)},
                            _ => (),
                        }
                    });
                } else {
                    if (0..=2).contains(&low) {
                        out[low as usize] = min as u32;
                    }
                }
                if high2bot {
                    bots.entry(high).and_modify(|x| {
                        match x {
                            Chips::Zero => {*x = Chips::One(max)},
                            Chips::One(y) => {*x = Chips::Two(*y, max)},
                            _ => (),
                        }
                    });
                } else {
                    if (0..=2).contains(&low) {
                        out[low as usize] = min as u32;
                    }
                }
                if !out.contains(&0) {
                    break;
                }
                commands.remove(index);
            };
        }
        println!("{}", out[0] * out[1] * out[2]);
        Some(())
    }
}