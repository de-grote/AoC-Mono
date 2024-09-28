#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        let input = std::include_str!("input.txt");
        let mut north_south: i32 = 0;
        let mut east_west: i32 = 0;
        let mut looking: u8 = 0;

        for direction in input.split(", ") {
            match direction.chars().next()? {
                'R' => {
                    looking = (looking + 1) % 4;
                }
                'L' => {
                    looking = (looking + 3) % 4;
                }
                _ => {
                    return None;
                }
            }

            let i: i32 = direction[1..].to_string().parse().ok()?;
            match looking {
                0 => {
                    north_south += i;
                }
                1 => {
                    east_west += i;
                }
                2 => {
                    north_south -= i;
                }
                3 => {
                    east_west -= i;
                }
                _ => {
                    return None;
                }
            };
        };
        println!("{}", north_south.abs() + east_west.abs());
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        let input = std::include_str!("input.txt");
        let mut north_south: i32 = 0;
        let mut east_west: i32 = 0;
        let mut looking: u8 = 0;
        let mut been = std::collections::HashSet::new();

        for direction in input.split(", ") {
            match direction.chars().next()? {
                'R' => {
                    looking = (looking + 1) % 4;
                }
                'L' => {
                    looking = (looking + 3) % 4;
                }
                _ => {
                    return None;
                }
            }

            let i: i32 = direction[1..].to_string().parse().ok()?;
            for _ in 0..i {
                match looking {
                    0 => {
                        north_south += 1;
                    }
                    1 => {
                        east_west += 1;
                    }
                    2 => {
                        north_south -= 1;
                    }
                    3 => {
                        east_west -= 1;
                    }
                    _ => {
                        return None;
                    }
                };
                if been.contains(&(north_south, east_west)) {
                    println!("{}", north_south.abs() + east_west.abs());
                    return Some(())
                }
                been.insert((north_south, east_west));
            }
        }
        None
    }
}