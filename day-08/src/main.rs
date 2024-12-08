#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::{fs, usize};

fn parse_antennas(inp: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let mut antennas = HashMap::<char, Vec<(i32, i32)>>::new();

    let mut row = 0i32;
    let mut width = 0i32;
    for line in inp.split("\n") {
        if !line.is_empty() {
            width = line.len().try_into().unwrap();
            for (col, c) in line.chars().enumerate() {
                let col: i32 = col.try_into().unwrap();
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert(Vec::<(i32, i32)>::new())
                        .push((row, col));
                }
            }
            row += 1;
        }
    }
    let height = row;

    (antennas, width, height)
}

fn part1(inp: &str, verbose: bool) -> usize {
    let (antennas, width, height) = parse_antennas(inp);

    //dbg!(&antennas);
    let names: Vec<&char> = antennas.keys().collect();
    let mut antinodes = HashSet::<(i32, i32)>::new();

    for name in names {
        let positions = antennas.get(name).unwrap();
        let perms: Vec<_> = positions.into_iter().permutations(2).collect();
        for perm in perms {
            let (a, b) = (perm.get(0).unwrap(), perm.get(1).unwrap());
            //dbg!(&a, &b);
            let an1 = (a.0 - (b.0 - a.0), a.1 - (b.1 - a.1));
            if (an1.0 >= 0) && (an1.0 < width) && (an1.1 >= 0) && (an1.1 < height) {
                //dbg!(r, c);
                antinodes.insert(an1);
                if verbose {
                    println!("antinode for {} @ ({:?})", name, an1)
                }
            }
            let an2 = (b.0 - (a.0 - b.0), b.1 - (a.1 - b.1));
            if (an2.0 >= 0) && (an2.0 < width) && (an2.1 >= 0) && (an2.1 < height) {
                //dbg!(r, c);
                antinodes.insert(an2);
                if verbose {
                    println!("antinode for {} @ ({:?})", name, an2)
                }
            }
        }
    }

    let result = antinodes.len();
    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn is_on_map(p: (i32, i32), width: i32, height: i32) -> bool {
    (p.0 >= 0) && (p.0 < width) && (p.1 >= 0) && (p.1 < height)
}

fn gen_all_antinodes(a: (i32, i32), b: (i32, i32), width: i32, height: i32) -> HashSet<(i32, i32)> {
    let mut antinodes = HashSet::<(i32, i32)>::new();
    let delta0 = b.0 - a.0;
    let delta1 = b.1 - a.1;

    for offset in [-1, 1] {
        let mut i = offset;
        loop {
            let an = (a.0 - i * delta0, a.1 - i * delta1);
            if is_on_map(an, width, height) {
                antinodes.insert(an);
                i += offset;
            } else {
                break;
            }
        }
    }

    antinodes
}

fn part2(inp: &str, verbose: bool) -> usize {
    let (antennas, width, height) = parse_antennas(inp);
    let names: Vec<&char> = antennas.keys().collect();
    let mut antinodes = HashSet::<(i32, i32)>::new();

    if verbose {
        println!("width: {}, height: {}", width, height);
    }
    for name in names {
        let positions = antennas.get(name).unwrap();
        //dbg!(&name, positions);
        let perms: Vec<_> = positions.into_iter().permutations(2).collect();
        if verbose {
            println!("key: {}, perms: {:?}", name, perms);
        }
        for perm in perms {
            let (a, b) = (perm.get(0).unwrap(), perm.get(1).unwrap());
            //dbg!(&a, &b);
            antinodes.extend(gen_all_antinodes(**a, **b, width, height));
            //antinodes.extend(gen_antinodes(**b, **a, width, height));
        }
    }

    let result = antinodes.len();
    if verbose {
        println!("result: {:?}", result);
    }
    result
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
    const TEXT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(14, part1(&text, true));
    }
    const TEXT2: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

    #[test]
    fn test_part2a() {
        let text = TEXT2.to_string();
        assert_eq!(9, part2(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(34, part2(&text, true));
    }
}
