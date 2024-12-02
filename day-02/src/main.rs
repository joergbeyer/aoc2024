#![allow(dead_code)]
use std::error::Error;
use std::fs;

fn all_increasing(v: &[i32]) -> bool {
    let mut prev = v[0];
    for idx in 1..v.len() {
        let cur = v[idx];
        if cur <= prev {
            return false;
        }
        prev = cur;
    }
    true
}
fn all_decreasing(v: &[i32]) -> bool {
    let mut prev = v[0];
    for idx in 1..v.len() {
        let cur = v[idx];
        if cur >= prev {
            return false;
        }
        prev = cur;
    }
    true
}
fn max_delta(v: &[i32], md: i32) -> bool {
    let mut prev = v[0];
    for idx in 1..v.len() {
        let cur = v[idx];
        let delta = (cur - prev).abs();
        if delta > md {
            return false;
        }
        prev = cur;
    }
    true
}

fn is_save(v: &[i32]) -> bool {
    (all_increasing(v) || all_decreasing(v)) && max_delta(v, 3)
}

fn part1(inp: &str, verbose: bool) -> i32 {
    let mut result = 0;

    for line in inp.split("\n") {
        if verbose {
            println!("line: {}", line);
        }
        if !line.is_empty() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|t| t.parse::<i32>().unwrap())
                .collect();
            if verbose {
                println!("numers: {:?}", numbers);
            }
            if is_save(&numbers) {
                result += 1;
            }
        }
    }

    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn part2(inp: &str, verbose: bool) -> i32 {
    let mut result = 0;

    for line in inp.split("\n") {
        if verbose {
            println!("line: {}", line);
        }
        if !line.is_empty() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|t| t.parse::<i32>().unwrap())
                .collect();
            if verbose {
                println!("numers: {:?}", numbers);
            }
            for i in 0..numbers.len() {
                let mut tmp = numbers.clone();
                tmp.remove(i);
                if is_save(&tmp) {
                    result += 1;
                    break;
                }
            }
        }
    }

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
    const TEXT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(2, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(4, part2(&text, true));
    }
}
