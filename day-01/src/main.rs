#![allow(dead_code)]
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn dist(a: i32, b: i32) -> i32 {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

fn part1(inp: &str, verbose: bool) -> i32 {
    let mut result = 0;

    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
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
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }
    left.sort();
    right.sort();
    let count = left.len();
    assert!(count == right.len());
    for i in 0..count {
        result += dist(left[i], right[i]);
    }
    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn freq_list(v: &[i32]) -> HashMap<i32, i32> {
    let frequencies = v.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    frequencies
}

fn sim_score(left: &[i32], right: &[i32], verbose: bool) -> i32 {
    let mut result = 0;
    let fr = freq_list(right);
    for num in left {
        result += num * fr.get(num).unwrap_or(&0);
    }
    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn part2(inp: &str, verbose: bool) -> i32 {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
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
                println!("numbers: {:?}", numbers);
            }
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }
    left.sort();
    right.sort();
    let result = sim_score(&left, &right, verbose);
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
    const TEXT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(11, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(31, part2(&text, true));
    }
}
