#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let mut length = 0;
        let mut s = String::new();
        let mut it = input.chars();
        while let Some(c) = it.next() {
            if c == '(' {
                let mut next = it.next()?;
                while next != 'x' {
                    s.push(next);
                    next = it.next()?;
                }
                let a: usize = s.parse().ok()?;
                s.clear();
                next = it.next()?;
                while next != ')' {
                    s.push(next);
                    next = it.next()?;
                }
                let b: usize = s.parse().ok()?;
                s.clear();
                length += a * b;
                for _ in 0..a {
                    it.next();
                }
            } else {
                length += 1
            }
        }
        println!("{}", length);
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let mut length = 0;
        let mut it = input.chars();
        fn f<K: Iterator<Item = char>>(c: char, it: &mut K) -> Option<usize> {
            if c == '(' {
                let mut s = String::new();
                let mut next = it.next()?;
                while next != 'x' {
                    s.push(next);
                    next = it.next()?;
                }
                let a: usize = s.parse().ok()?;
                s.clear();
                next = it.next()?;
                while next != ')' {
                    s.push(next);
                    next = it.next()?;
                }
                let b: usize = s.parse().ok()?;
                s.clear();
                let mut v = Vec::new();
                for _ in 0..a {
                    v.push(it.next()?);
                }
                let mut it2 = v.into_iter();
                let mut d = 0;
                while let Some(c) = it2.next() {
                    d += f(c, &mut it2)?;
                }
                Some(d * b)
            } else {
                Some(1)
            }
        }
        while let Some(c) = it.next() {
            length += f(c, &mut it)?;
        }
        println!("{}", length);
        Some(())
    }
}