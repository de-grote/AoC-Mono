use crate::prelude::*;

pub fn part1(input: &str) -> Answer {
    let (_, connections) = parse(input)?;

    let mut graph: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();

    for (a, b) in connections {
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let mut neighbors: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (&k, v) in graph.range("ta"..="tz") {
        for &node in v {
            neighbors.entry(node).or_default().insert(k);
        }
    }

    let mut combs: BTreeSet<[&str; 3]> = BTreeSet::new();

    for (&neighbor, from) in neighbors.iter() {
        let n = graph.get(neighbor).unwrap();
        for &node2 in n {
            for &node3 in graph.get(node2).unwrap() {
                if from.contains(node3) {
                    let mut c = [neighbor, node2, node3];
                    c.sort();
                    combs.insert(c);
                }
            }
        }
    }

    Ok(combs.len().to_string())
}

pub fn part2(input: &str) -> Answer {
    let (_, connections) = parse(input)?;

    let mut graph: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();

    for (a, b) in connections {
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let res = largest_connected_subgraph(&graph, BTreeSet::new(), &mut BTreeSet::new());

    Ok(res.into_iter().join(","))
}

fn largest_connected_subgraph<'a>(
    graph: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    subgraph: BTreeSet<&'a str>,
    cache: &mut BTreeSet<BTreeSet<&'a str>>,
) -> BTreeSet<&'a str> {
    if cache.contains(&subgraph) {
        return BTreeSet::new();
    }
    let to_add: BTreeSet<&str> = subgraph
        .iter()
        .fold(graph.keys().copied().collect(), |acc, &k| {
            &acc & graph.get(k).unwrap()
        });
    let mut max = subgraph.clone();
    for adding in to_add {
        let mut new_set = subgraph.clone();
        new_set.insert(adding);
        let res = largest_connected_subgraph(graph, new_set, cache);
        if res.len() > max.len() {
            max = res;
        }
    }
    cache.insert(max.clone());
    max
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(line_ending, separated_pair(alpha1, tag("-"), alpha1))(input)
}
