#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        let input = include_bytes!("input.txt");
        let mut v;
        let mut s = String::new();
        for i in 0u32.. {
            v = input.to_vec();
            v.extend(i.to_string().as_bytes().to_owned());
            let d = *md5::compute(v);
            if d[0] == 0 && d[1] == 0 && d[2] & 0xF0 == 0 {
                s.push(char::from_digit(d[2] as u32, 16)?);
                if s.len() == 8 {
                    println!("{}", s);
                    return Some(())
                }
            }
        }
        None
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        let input = include_bytes!("input.txt");
        let mut v;
        let mut s: [u8; 8] = Default::default();
        let mut finds: [bool; 8] = Default::default();
        for i in 0u32.. {
            v = input.to_vec();
            v.extend(i.to_string().as_bytes());
            let d = *md5::compute(v);
            if d[0] == 0 && d[1] == 0 && d[2] <= 7 && finds[d[2] as usize] == false {
                s[d[2] as usize] = (d[3] & 0xF0) >> 4;
                finds[d[2] as usize] = true;
                if !finds.contains(&false) {
                    for c in s {
                        print!("{}", char::from_digit(c as u32, 16)?)
                    }
                    println!();
                    return Some(())
                }
            }
        }
        None
    }
}