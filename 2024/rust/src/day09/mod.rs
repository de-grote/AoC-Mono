use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, disc) = parse(input)?;
    let mut buffer: Vec<Option<u32>> = Vec::new();
    let mut file = true;
    let mut block = 0;
    for i in disc {
        if file {
            buffer.extend(std::iter::repeat_n(Some(block), i as usize));
        } else {
            buffer.extend(std::iter::repeat_n(None, i as usize));
            block += 1;
        }
        file = !file;
    }

    let mut i = 0;
    while i < buffer.len() {
        if buffer[i].is_some() {
            i += 1;
            continue;
        }
        if let Some(v) = buffer.pop().flatten() {
            buffer[i] = Some(v);
            i += 1;
        }
    }

    let res: u64 = buffer
        .into_iter()
        .flatten()
        .zip(0..)
        .map(|(a, b)| a as u64 * b)
        .sum();

    Ok(res.to_string())
}

#[derive(Debug, Clone, Copy)]
enum SpaceOrId {
    Space { size: u32 },
    Id { size: u32, id: u32 },
}

pub fn part2(input: &str) -> Answer {
    let (_, disc) = parse(input)?;
    let mut buffer: Vec<SpaceOrId> = Vec::new();
    let mut file = true;
    let mut block = 0;
    for i in disc {
        if file {
            buffer.push(SpaceOrId::Id { size: i, id: block });
        } else {
            if i != 0 {
                buffer.push(SpaceOrId::Space { size: i });
            }
            block += 1;
        }
        file = !file;
    }

    if let Some(SpaceOrId::Space { size: _ }) = buffer.last() {
        block -= 1;
        buffer.pop();
    }

    let mut block = block as i32;

    while block >= 0 {
        let (mut from_pos, (_, needed_size)) = buffer
            .iter()
            .map(|&v| match v {
                SpaceOrId::Id { size, id } => (id as i32, size),
                _ => (-1, 0),
            })
            .find_position(|&v| v.0 == block)
            .unwrap();
        if let Some((to_pos, &s)) = buffer.iter().find_position(|&&v| match v {
            SpaceOrId::Space { size } => size >= needed_size,
            SpaceOrId::Id { .. } => false,
        }) {
            if to_pos < from_pos {
                let SpaceOrId::Space { size: actual_size } = s else {
                    unreachable!()
                };
                buffer[to_pos] = SpaceOrId::Id {
                    size: needed_size,
                    id: block as u32,
                };
                let mut new_size = needed_size;
                if let Some(SpaceOrId::Space { size }) = buffer.get(from_pos + 1) {
                    new_size += size;
                    buffer.remove(from_pos + 1);
                }
                if let Some(SpaceOrId::Space { size }) = buffer.get(from_pos.wrapping_sub(1)) {
                    new_size += size;
                    from_pos -= 1;
                    buffer.remove(from_pos);
                }
                buffer[from_pos] = SpaceOrId::Space { size: new_size };
                if needed_size != actual_size {
                    buffer.insert(
                        to_pos + 1,
                        SpaceOrId::Space {
                            size: actual_size - needed_size,
                        },
                    );
                }
            }
        }
        block -= 1;
    }

    let res: u64 = buffer
        .into_iter()
        .flat_map(|v| {
            let (size, v) = match v {
                SpaceOrId::Space { size } => (size, 0),
                SpaceOrId::Id { size, id } => (size, id),
            };
            std::iter::repeat_n(v, size as usize)
        })
        .zip(0..)
        .map(|(a, b)| a as u64 * b)
        .sum();

    Ok(res.to_string())
}

#[allow(dead_code)]
fn print_buffer(v: &[SpaceOrId]) {
    let s = v
        .iter()
        .map(|x| match x {
            SpaceOrId::Space { size } => {
                std::iter::repeat_n('.', *size as usize).collect::<String>()
            }
            SpaceOrId::Id { size, id } => {
                std::iter::repeat_n(id.to_string().chars().next().unwrap(), *size as usize)
                    .collect::<String>()
            }
        })
        .join("");
    println!("{}", s);
}

fn parse(input: &str) -> IResult<&str, Vec<u32>> {
    many1(satisfy(|c| c.is_ascii_digit()).map(|d| d.to_digit(10).unwrap()))(input)
}
