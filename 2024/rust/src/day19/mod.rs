use std::ptr::addr_of;

use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, (towels, patterns)) = parse(input)?;

    let mut trie = Trie::default();
    for pattern in &towels {
        let mut t = &mut trie;
        for &p in pattern {
            let field = match p {
                Color::White => &mut t.white,
                Color::Blue => &mut t.blue,
                Color::Black => &mut t.black,
                Color::Red => &mut t.red,
                Color::Green => &mut t.green,
            };
            if field.is_none() {
                *field = Some(Box::new(Trie::default()))
            }
            t = field.as_mut().unwrap();
        }
        t.is_end = true;
    }

    let res = patterns
        .into_iter()
        .filter(|pattern| find_towels(&trie, pattern, &mut HashSet::new()))
        .count();

    Ok(res.to_string())
}

fn find_towels(towels: &Trie, pattern: &[Color], cache: &mut HashSet<usize>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if !cache.insert(addr_of!(pattern[0]) as usize) {
        return false;
    }
    let mut t = towels;
    let mut reqs = Vec::new();
    for (i, c) in pattern.iter().enumerate() {
        let field = match c {
            Color::White => t.white.as_ref(),
            Color::Blue => t.blue.as_ref(),
            Color::Black => t.black.as_ref(),
            Color::Red => t.red.as_ref(),
            Color::Green => t.green.as_ref(),
        };
        let Some(f) = field else {
            break;
        };
        t = f.as_ref();
        if t.is_end {
            reqs.push(&pattern[i + 1..]);
        }
    }
    reqs.into_iter()
        .rev()
        .any(move |p| find_towels(towels, p, cache))
}

pub fn part2(input: &str) -> Answer {
    let (_, (towels, patterns)) = parse(input)?;

    let mut trie = Trie::default();
    for pattern in &towels {
        let mut t = &mut trie;
        for &p in pattern {
            let field = match p {
                Color::White => &mut t.white,
                Color::Blue => &mut t.blue,
                Color::Black => &mut t.black,
                Color::Red => &mut t.red,
                Color::Green => &mut t.green,
            };
            if field.is_none() {
                *field = Some(Box::new(Trie::default()))
            }
            t = field.as_mut().unwrap();
        }
        t.is_end = true;
    }

    let res: usize = patterns
        .into_iter()
        .map(|pattern| find_towels2(&trie, &pattern, &mut HashMap::new()))
        .sum();

    Ok(res.to_string())
}

fn find_towels2(towels: &Trie, pattern: &[Color], cache: &mut HashMap<usize, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&v) = cache.get(&(addr_of!(pattern[0]) as usize)) {
        return v;
    }
    let mut t = towels;
    let mut reqs = Vec::new();
    for (i, c) in pattern.iter().enumerate() {
        let field = match c {
            Color::White => t.white.as_ref(),
            Color::Blue => t.blue.as_ref(),
            Color::Black => t.black.as_ref(),
            Color::Red => t.red.as_ref(),
            Color::Green => t.green.as_ref(),
        };
        let Some(f) = field else {
            break;
        };
        t = f.as_ref();
        if t.is_end {
            reqs.push(&pattern[i + 1..]);
        }
    }
    let res = reqs
        .into_iter()
        .map(|p| find_towels2(towels, p, cache))
        .sum();
    cache.insert(addr_of!(pattern[0]) as usize, res);
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Trie {
    is_end: bool,
    white: Option<Box<Trie>>,
    blue: Option<Box<Trie>>,
    black: Option<Box<Trie>>,
    red: Option<Box<Trie>>,
    green: Option<Box<Trie>>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    satisfy(|x| ['w', 'b', 'u', 'r', 'g'].contains(&x))
        .map(|x| match x {
            'w' => Color::White,
            'b' => Color::Black,
            'u' => Color::Blue,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => unreachable!(),
        })
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<Color>>, Vec<Vec<Color>>)> {
    separated_pair(
        separated_list1(tag(", "), many1(parse_color)),
        multispace1,
        separated_list1(line_ending, many1(parse_color)),
    )(input)
}
