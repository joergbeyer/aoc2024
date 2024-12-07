#![allow(dead_code)]
use std::error::Error;
use std::fs;

fn parse_line(line: &str) -> (i128, Vec<i128>) {
    let Some((lhs, rhs)) = line.split_once(":") else {
        todo!()
    };
    let target = lhs.parse::<i128>().unwrap();
    let values: Vec<_> = rhs
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect();

    (target, values)
}

fn op_plus(a: i128, b: i128) -> i128 {
    a + b
}
fn op_mult(a: i128, b: i128) -> i128 {
    a * b
}

fn op_concat(a: i128, b: i128) -> i128 {
    let s = format!("{}{}", a, b);
    s.parse::<i128>().expect("should be 2 numbers")
}

fn is_solvable(
    target: &i128,
    result: i128,
    vals: &[i128],
    ops: &Vec<&dyn Fn(i128, i128) -> i128>,
) -> bool {
    (result <= *target) && {
        if vals.is_empty() {
            result == *target
        } else {
            let head = vals[0];
            ops.iter()
                .any(|op| is_solvable(target, op(result, head), &vals[1..], ops))
        }
    }
}

fn part1(inp: &str, verbose: bool) -> i128 {
    let mut result = 0;

    for line in inp.split("\n") {
        if verbose {
            println!("line: {}", line);
        }
        if !line.is_empty() {
            let (target, values) = parse_line(line);

            let ops: Vec<&dyn Fn(i128, i128) -> i128> = vec![&op_plus, &op_mult];
            if is_solvable(&target, 0, &values, &ops) {
                result += target;
            }
        }
    }

    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn part2(inp: &str, verbose: bool) -> i128 {
    let mut result = 0;

    for line in inp.split("\n") {
        if verbose {
            println!("line: {}", line);
        }
        if !line.is_empty() {
            let (target, values) = parse_line(line);

            let ops: Vec<&dyn Fn(i128, i128) -> i128> = vec![&op_plus, &op_mult, &op_concat];
            if is_solvable(&target, 0, &values, &ops) {
                result += target;
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
    const TEXT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(3749, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(11387, part2(&text, true));
    }
}
