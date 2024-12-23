#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn read_graph(inp: &str, verbose: bool) -> HashMap<String, HashSet<String>> {
    let mut edges = HashMap::<String, HashSet<String>>::new();
    for line in inp.split("\n") {
        if verbose {
            println!("line: {}", line);
        }

        if !line.is_empty() {
            let (lhs, rhs) = line.split_once('-').unwrap();
            //dbg!(&lhs, &rhs);
            assert!(lhs != rhs);
            edges
                .entry(lhs.to_string())
                .or_default()
                .insert(rhs.to_string());
            edges
                .entry(rhs.to_string())
                .or_default()
                .insert(lhs.to_string());
        }
    }

    edges
}

fn part1(inp: &str, verbose: bool) -> usize {
    let mut result = 0;

    let edges = read_graph(inp, verbose);

    let mut groups_of_3 = HashSet::<Vec<String>>::new();
    for (node, neighbors) in &edges {
        for neighor in neighbors {
            let others = edges.get(neighor).unwrap();
            for other in others {
                if neighbors.contains(other) {
                    let mut group = vec![node.clone(), neighor.clone(), other.to_string()];
                    group.sort();
                    if verbose {
                        println!("group: {}, {}, {}", &group[0], &group[1], &group[2]);
                    }
                    groups_of_3.insert(group);
                }
            }
        }
    }
    for g in &groups_of_3 {
        if g[0].starts_with("t") || g[1].starts_with("t") || g[2].starts_with("t") {
            if verbose {
                println!("{},{},{}", g[0], g[1], g[2]);
            }
            result += 1;
        }
    }
    result
}

fn part2(inp: &str, verbose: bool) -> String {
    let edges = read_graph(inp, verbose);
    let mut largest_group = vec![];

    for node in edges.keys() {
        let mut group = vec![node];
        if let Some(neighbors) = edges.get(node) {
            for neighbor in neighbors {
                if group
                    .iter()
                    .all(|n| edges.get(neighbor).unwrap().contains(*n))
                {
                    group.push(neighbor);
                }
            }

            if group.len() > largest_group.len() {
                largest_group = group;
            }
        }
    }
    largest_group.sort();
    largest_group
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join(",")
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = fs::read_to_string("input1.txt")?;

    let res1 = part1(&inp, false);
    println!("result from part1: {}", res1);

    let res2 = part2(&inp, false);
    println!("result from part2: {}", res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEXT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(7, part1(&text, false));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!("co,de,ka,ta", part2(&text, false));
    }
}
