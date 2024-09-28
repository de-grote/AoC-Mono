#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let mut x: i8 = 1;
        let mut y: i8 = 1;
        let mut out = String::new();
        for i in input.split("\n").map(|a| a.trim_end()) {
            for c in i.chars() {
                match c {
                    'U' => {
                        if y != 0 {
                            y -= 1
                        }
                    }
                    'D' => {
                        if y != 2 {
                            y += 1
                        }
                    }
                    'L' => {
                        if x != 0 {
                            x -= 1
                        }
                    }
                    'R' => {
                        if x != 2 {
                            x += 1
                        }
                    }
                    _ => return None
                }
            }
            out += &(x + 3 * y + 1).to_string();
        }
        println!("{}", out);
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        use std::collections::HashMap;
        let input = include_str!("input.txt");
        let mut x: i8 = 0;
        let mut y: i8 = 2;
        let mut out = String::new();
        let valid = HashMap::from([((0, 2), '5'), ((1, 1), '2'), ((1, 2), '6'), ((1, 3), 'A'), ((2, 0), '1'), ((2, 1), '3'), ((2, 2), '7'), ((2, 3), 'B'), ((2, 4), 'D'), ((3, 1), '4'), ((3, 2), '8'), ((3, 3), 'C'), ((4, 2), '9')]);
        for i in input.split("\n").map(|a| a.trim_end()) {
            for c in i.chars() {
                match c {
                    'U' => {
                        if valid.contains_key(&(x, y - 1)) {
                            y -= 1
                        }
                    }
                    'D' => {
                        if valid.contains_key(&(x, y + 1)) {
                            y += 1
                        }
                    }
                    'L' => {
                        if valid.contains_key(&(x - 1, y)) {
                            x -= 1
                        }
                    }
                    'R' => {
                        if valid.contains_key(&(x + 1, y)) {
                            x += 1
                        }
                    }
                    _ => return None
                }
            }
            out.push(valid.get(&(x, y))?.clone());
        }
        println!("{}", out);
        Some(())
    }
}