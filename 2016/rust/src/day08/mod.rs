#![allow(dead_code)]

pub mod part1 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let mut screen: [[bool; 50]; 6] = unsafe {std::mem::zeroed()};
        for i in input.split("\n").map(|s| s.trim_end()) {
            if &i[1..2] == "e" { // rect
                let (x, y) = i[5..].split_once('x')?;
                let x: usize = x.to_string().parse().ok()?;
                let y: usize = y.to_string().parse().ok()?;
                for j in &mut screen[..y] {
                    for k in 0..x {
                        j[k] = true;
                    }
                }
            } else {
                if &i[7..8] == "r" { // row
                    let (row, amount) = i[13..].split_once(" by ")?;
                    let row: usize = row.parse().ok()?;
                    let amount: usize = amount.parse().ok()?;
                    screen[row].rotate_right(amount);                   
                } else { // column
                    let (column, amount) = i[16..].split_once(" by ")?;
                    let column: usize = column.parse().ok()?;
                    let amount: usize = amount.parse().ok()?;
                    let mut b = false;
                    for _ in 0..amount {
                        for k in 0..6usize {
                            let temp = screen[k][column];
                            screen[k][column] = b;
                            b = temp;
                        }
                        screen[0][column] = b;
                    }
                }
            }
        }
        let mut int = 0;
        screen.map(|r| r.map(|b| if b {int += 1}));
        println!("{}", int);
        Some(())
    }
}

pub mod part2 {
    pub fn solve() -> Option<()> {
        let input = include_str!("input.txt");
        let mut screen: [[bool; 50]; 6] = unsafe {std::mem::zeroed()};
        for i in input.split("\n").map(|s| s.trim_end()) {
            if &i[1..2] == "e" { // rect
                let (x, y) = i[5..].split_once('x')?;
                let x: usize = x.to_string().parse().ok()?;
                let y: usize = y.to_string().parse().ok()?;
                for j in &mut screen[..y] {
                    for k in 0..x {
                        j[k] = true;
                    }
                }
            } else {
                if &i[7..8] == "r" { // row
                    let (row, amount) = i[13..].split_once(" by ")?;
                    let row: usize = row.parse().ok()?;
                    let amount: usize = amount.parse().ok()?;
                    screen[row].rotate_right(amount);                   
                } else { // column
                    let (column, amount) = i[16..].split_once(" by ")?;
                    let column: usize = column.parse().ok()?;
                    let amount: usize = amount.parse().ok()?;
                    let mut b = false;
                    for _ in 0..amount {
                        for k in 0..6usize {
                            let temp = screen[k][column];
                            screen[k][column] = b;
                            b = temp;
                        }
                        screen[0][column] = b;
                    }
                }
            }
        }
        screen.map(|r| {r.map(|b| if b {print!("#")} else {print!(" ")}); println!()});
        Some(())
    }
}