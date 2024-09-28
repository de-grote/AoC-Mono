#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut a: [HashMap<char, u8>; 8] = Default::default();
        for s in input.split("\n").map(|s| s.trim_end()) {
            for (i, c) in s.chars().enumerate() {
                a[i].entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        let it = a.into_iter().filter_map(|x| {
            let m = *x.values().max()?;
            for (k, v) in x {
                if v == m {
                    return Some(k);
                }
            }
            None
        });
        println!("{}", String::from_iter(it));
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut a: [HashMap<char, u8>; 8] = Default::default();
        for s in input.split("\n").map(|s| s.trim_end()) {
            for (i, c) in s.chars().enumerate() {
                a[i].entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
        }
        let it = a.into_iter().filter_map(|x| {
            let m = *x.values().min()?;
            for (k, v) in x {
                if v == m {
                    return Some(k);
                }
            }
            None
        });
        println!("{}", String::from_iter(it));
        Some(())
    }
}