#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut counter: HashMap<char, i8> = HashMap::new();
        let mut out: u32 = 0;
        for i in input.split("\n").map(|s| s.trim_end()) {
            let checksum = i[i.len()-6..i.len()-1].to_string();
            let id: u32 = i[i.len()-10..i.len()-7].parse().ok()?;
            for l in i[..i.len()-11].chars() {
                if l != '-' {
                    counter.entry(l).and_modify(|f| *f += 1).or_insert(1);
                }
            }
            let mut l: Vec<_> = counter.drain().map(|(k, v)| (-v, k)).collect();
            l.sort();
            let s: String = l[..5].iter().map(|(_, v)| v).collect();
            if s == checksum {
                out += id;
            }
        }
        println!("{}", out);
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut counter: HashMap<char, i8> = HashMap::new();
        for i in input.split("\n").map(|s| s.trim_end()) {
            let checksum = i[i.len()-6..i.len()-1].to_string();
            let id: u32 = i[i.len()-10..i.len()-7].parse().ok()?;
            let word = &i[..i.len()-11];
            for l in word.chars() {
                if l != '-' {
                    counter.entry(l).and_modify(|f| *f += 1).or_insert(1);
                }
            }
            let mut l: Vec<_> = counter.drain().map(|(k, v)| (-v, k)).collect();
            l.sort();
            let s: String = l[..5].iter().map(|(_, v)| v).collect();
            if s == checksum {
                let mut name = String::new();
                for c in word.chars() {
                    if c == '-' {
                        name += " ";
                    } else {
                        name.push(char::from(((u32::from(c) - 97 + id) % 26 + 97) as u8));
                    }
                }
                println!("{}: {}", name, id);
            }
        }
        Some(())
    }
}