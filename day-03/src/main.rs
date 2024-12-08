#![allow(dead_code)]
use regex::Regex;
use std::error::Error;
use std::fs;

fn part1(inp: &str, verbose: bool) -> usize {
    let mut result = 0;
    let re = Regex::new(r#"mul\((\d*),(\d*)\)"#).unwrap();
    for cap in re.captures_iter(inp) {
        let groups = (cap.get(1), cap.get(2));
        match groups {
            (Some(lhs), Some(rhs)) => {
                if verbose {
                    println!("[lhs={}, rhs={}]", lhs.as_str(), rhs.as_str());
                }
                Some(
                    lhs.as_str().parse::<usize>().unwrap_or(0)
                        * rhs.as_str().parse::<usize>().unwrap_or(0),
                )
            }
            _ => None,
        }
    });

    //exprs.for_each(|e| result += e);
    for e in exprs {
        if verbose {
            println!("{:?}", e);
        }
        result += e;
    }
    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn part2(inp: &str, verbose: bool) -> usize {
    let mut result = 0;
    let re =
        Regex::new(r"(?:mul\((?<lhs>\d+),(?<rhs>\d+)\))|(?<on>do\(\))|(?<off>don't\(\))").unwrap();

    let mut active = true;
    for cap in re.captures_iter(&inp) {
        if cap.name("on").is_some() {
            active = true;
            continue;
        }
        if cap.name("off").is_some() {
            active = false;
            continue;
        }
        if active {
            let (num1, num2) = (&cap["lhs"], &cap["rhs"]);
            result += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
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
//result from part1: 164730528
//result from part2: 70478672

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEXT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEXT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        let text = TEXT1.to_string();
        assert_eq!(161, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT2.to_string();
        assert_eq!(48, part2(&text, true));
    }
}
