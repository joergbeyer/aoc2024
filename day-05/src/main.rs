#![allow(dead_code)]
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn rule_is_ok(a: usize, b: usize, rule: &String) -> bool {
    let Some(order) = rule.split_once("|") else {
        todo!()
    };
    //dbg!(&order);
    let (before, after) = order;
    let before = before.parse::<usize>().unwrap();
    let after = after.parse::<usize>().unwrap();
    if (b == before) && (a == after) {
        return false;
    }
    //println!("a: {}, b: {}")

    true
}

fn handle_line(line: &str, rules: &Vec<String>, verbose: bool) -> (bool, Vec<usize>) {
    let mut resorted = false;
    println!("handle: {}", line);

    let mut nums: Vec<_> = line
        .split(",")
        .map(|e| e.parse::<usize>().unwrap())
        .collect();

    let mut rel_rules = HashMap::<usize, Vec<String>>::new();
    for rule in rules {
        let tmp: Vec<usize> = rule
            .split("|")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let lhs = tmp.get(0).unwrap();
        let rhs = tmp.get(1).unwrap();
        if nums.contains(lhs) {
            rel_rules
                .entry(*lhs)
                .or_insert_with(Vec::new)
                .push(rule.clone());
        }
        if nums.contains(rhs) {
            rel_rules
                .entry(*rhs)
                .or_insert_with(Vec::new)
                .push(rule.clone());
        }
    }
    if verbose {
        println!("\trel rules: {:?}", rel_rules);
        println!("\tnums: {:?}", nums);
    }
    loop {
        let mut updated = false;
        'outer: for i in 1..nums.len() {
            let a = nums.get(i - 1).unwrap();
            let b = nums.get(i).unwrap();
            if verbose {
                println!("\t\tpair: {}, {}", a, b);
            }
            for rule in rel_rules.get(a).unwrap() {
                if rule_is_ok(*a, *b, rule) {
                    if verbose {
                        println!("\t\t\trule: OK  {:?}", rule,);
                    }
                } else {
                    if verbose {
                        println!("\t\t\trule: ERR {:?}", rule,);
                    }
                    updated = true;
                    resorted = true;
                    nums.swap(i - 1, i);
                    break 'outer;
                }
            }
        }
        if !updated {
            break;
        }
    }
    (resorted, nums)
}

fn part1(inp: &str, verbose: bool) -> usize {
    let mut result = 0;
    let (top, bottom) = split_str_once(inp, "\n\n");

    let rules: Vec<String> = top.split("\n").map(|x| x.to_string()).collect();
    let pages: Vec<_> = bottom.split("\n").collect();

    println!("ordering rules: {:?}", rules);
    println!("page numbers: {:?}", pages);
    for line in pages {
        if !line.is_empty() {
            let resorted;
            let sorted: Vec<usize>;
            (resorted, sorted) = handle_line(line, &rules, false);
            if !resorted {
                let v: Vec<_> = line.split(",").map(|e| e.parse::<usize>()).collect();
                let midx: usize = v.len() / 2;
                let m = v.iter().nth(midx).unwrap();
                println!("m: {:?}", m);
                if m.is_ok() {
                    result += m.clone().unwrap();
                }
            }
            println!("sorted: {:?}", sorted);
        }
    }
    result
}

fn split_str_once(inp: &str, splitter: &str) -> (String, String) {
    let sections: Vec<_> = inp.split(splitter).collect();
    let left = sections.get(0).unwrap().to_string();
    let right = sections.get(1).unwrap().to_string();

    (left, right)
}

fn part2(inp: &str, verbose: bool) -> usize {
    let mut result = 0;

    let (top, bottom) = split_str_once(inp, "\n\n");

    let ord_rules: Vec<String> = top.split("\n").map(|x| x.to_string()).collect();
    let page_numbers: Vec<_> = bottom.split("\n").collect();
    println!("ordering rules: {:?}", ord_rules);
    println!("page numbers: {:?}", page_numbers);
    for line in page_numbers {
        if !line.is_empty() {
            let resorted;
            let v: Vec<usize>;
            (resorted, v) = handle_line(line, &ord_rules, false);
            if resorted {
                let midx: usize = v.len() / 2;
                let m = v.iter().nth(midx).unwrap();
                println!("m: {:?}", m);
                result += m;
            }
            println!("sorted: {:?}", v);
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = fs::read_to_string("input1.txt")?;

    //let res1 = part1(&inp, false);
    //println!("result from part1: {}", res1);

    let res2 = part2(&inp, false);
    println!("result from part2: {}", res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEXT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(143, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(123, part2(&text, true));
    }
}
