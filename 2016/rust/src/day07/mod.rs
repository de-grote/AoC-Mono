#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let re = fancy_regex::Regex::new(r"(\w)(?!\1)(\w)\2\1").ok()?;
        let mut out: u32 = 0;
        let mut b;
        for i in input.split("\n").map(|s| s.trim_end()) {
            b = false;
            for (u, j) in i.split(|x| x == '[' || x == ']').enumerate() {
                if re.is_match(j).ok()? {
                    if u & 1 == 0 {
                        b = true;
                    } else {
                        b = false;
                        break
                    }
                }
            }
            if b {
                out += 1
            }
        }
        println!("{}", out);
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let re = fancy_regex::Regex::new(r"(\w)(?!\1)\w\1").ok()?;
        let mut seen = std::collections::HashSet::new();
        let mut out: u32 = 0;
        for i in input.split("\n").map(|s| s.trim_end()) {
            seen.clear();
            'm: for (u, j) in i.split(|x| x == '[' || x == ']').enumerate() {
                let mut pos = 0;
                loop {
                    let res = re.find_from_pos(j, pos).ok()?;
                    if let Some(r) = res {
                        pos = r.start() + 1;
                        let mut it = r.as_str().chars();
                        let a = match u & 1 == 0 {
                            true => {
                                let x = it.next()?;
                                let y = it.next()?;
                                ((x, y), true)
                            }
                            false => {
                                let x = it.next()?;
                                let y = it.next()?;
                                ((y, x), false)
                            }
                        };
                        if seen.contains(&(a.0, !a.1)) {
                            out += 1;
                            break 'm
                        }
                        seen.insert(a);
                    } else {
                        break
                    }
                }
            }
        }
        println!("{}", out);
        Some(())
    }
}