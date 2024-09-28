#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        use std::cmp::max;
        let mut out: u32 = 0;
        let input = include_str!("input.txt");
        for i in input.split("\n") {
            let a: u16 = i[2..5].trim_start().parse().ok()?;
            let b: u16 = i[7..10].trim_start().parse().ok()?;
            let c: u16 = i[12..15].trim_start().parse().ok()?;

            if a + b + c > max(max(a, b), c) * 2 {
                out += 1
            }
        }
        println!("{}", out);
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        use std::cmp::max;
        let mut out: u32 = 0;
        let input = include_str!("input.txt");
        let mut it = input.split("\n");
        while let Some(mut i) = it.next() {
            let a: u16 = i[2..5].trim_start().parse().ok()?;
            let b: u16 = i[7..10].trim_start().parse().ok()?;
            let c: u16 = i[12..15].trim_start().parse().ok()?;

            i = it.next()?;

            let d: u16 = i[2..5].trim_start().parse().ok()?;
            let e: u16 = i[7..10].trim_start().parse().ok()?;
            let f: u16 = i[12..15].trim_start().parse().ok()?;

            i = it.next()?;

            let g: u16 = i[2..5].trim_start().parse().ok()?;
            let h: u16 = i[7..10].trim_start().parse().ok()?;
            let j: u16 = i[12..15].trim_start().parse().ok()?;

            if a + d + g > max(max(a, d), g) * 2 {
                out += 1
            }

            if b + e + h > max(max(b, e), h) * 2 {
                out += 1
            }

            if c + f + j > max(max(c, f), j) * 2 {
                out += 1
            }
        }
        println!("{}", out);
        Some(())
    }
}