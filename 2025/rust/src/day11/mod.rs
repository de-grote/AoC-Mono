use crate::prelude::*;

pub fn part1(input: &str) -> Answer<u32> {
    let (_, paths) = parse(input)?;

    let graph: HashMap<&str, HashSet<&str>> = paths
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    let mut next = HashSet::from_iter(["you"]);
    let mut prev: HashMap<&str, u32> = HashMap::new();
    prev.insert("you", 1);
    while !next.is_empty() {
        let mut new = HashSet::new();
        for node in next {
            let Some(neighbours) = graph.get(node) else {
                continue;
            };
            for &neighbour in neighbours {
                *prev.entry(neighbour).or_default() += *prev.entry(node).or_default();
                new.insert(neighbour);
            }
        }
        next = new;
    }

    Ok(prev["out"])
}

pub fn part2(input: &str) -> Answer<u64> {
    let (_, paths) = parse(input)?;

    let graph: HashMap<&str, HashSet<&str>> = paths
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    let fft1 = bfs(&graph, "svr", "fft");
    let fft2 = bfs(&graph, "fft", "dac");
    let fft3 = bfs(&graph, "dac", "out");
    let dac1 = bfs(&graph, "svr", "dac");
    let dac2 = bfs(&graph, "dac", "fft");
    let dac3 = bfs(&graph, "fft", "out");
    // dbg!(fft1, fft2, fft3, dac1, dac2, dac3);

    Ok(fft1 * fft2 * fft3 + dac1 * dac2 * dac3)
}

fn bfs(graph: &HashMap<&str, HashSet<&str>>, start: &str, end: &str) -> u64 {
    let mut next = HashSet::from_iter([start]);
    let mut prev: HashMap<&str, u64> = HashMap::new();
    prev.insert(start, 1);
    let mut res = 0;
    while !next.is_empty() {
        let mut new = HashSet::new();
        let mut to_add: HashMap<&str, u64> = HashMap::new();
        for node in next {
            let Some(neighbours) = graph.get(node) else {
                continue;
            };
            for &neighbour in neighbours {
                let a = *prev.entry(node).or_default();
                let r = to_add.entry(neighbour).or_default();

                *r += a;
                new.insert(neighbour);
            }
        }
        res += to_add.get(end).unwrap_or(&0);
        prev = to_add;
        next = new;
    }
    res
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(
        multispace1,
        separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
    )
    .parse(input)
}
