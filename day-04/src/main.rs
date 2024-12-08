#![allow(dead_code)]
use std::error::Error;
use std::fs;

// retrun true if t1 fully contains t2)
fn fully_contains(t1: (i32, i32), t2: (i32, i32)) -> bool {
    (t1.0 <= t2.0) && (t1.1 >= t2.1)
}

fn parse_ranges(text: &str, verbose: bool) -> i32 {
    let mut result = 0;
    for line in text.split("\n") {
        // eg line := "2-4,6,8"
        if verbose {
            println!("line: {}", line);
        }
        let ranges: Vec<_> = line.split(",").collect();
        println!("ranges: {:?}", ranges);
        // ranges := ["2-4", "6-8"]

        let mut segs = Vec::<(i32, i32)>::new();
        for range in ranges {
            let seg: Vec<_> = range
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            segs.push((seg[0], seg[1]));
        }
        println!("segs: {:?}", segs);
        // segs := [(2,4), (6,8)]
        if fully_contains(segs[0], segs[1]) || fully_contains(segs[1], segs[0]) {
            result += 1;
        }
    }

    result
}

fn parse_ranges2(text: &str, verbose: bool) -> i32 {
    let mut result = 0;
    for line in text.split("\n") {
        // eg line := "2-4,6,8"
        if verbose {
            println!("line: {}", line);
        }

        let mut segs = Vec::<(i32, i32)>::new();
        for range in line.split(",") {
            let seg: Vec<_> = range
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            segs.push((seg[0], seg[1]));
        }
        println!("segs: {:?}", segs);
        // segs := [(2,4), (6,8)]
        if fully_contains(segs[0], segs[1]) || fully_contains(segs[1], segs[0]) {
            result += 1;
        }
    }

    result
}
fn parse_ranges3(text: &str, _verbose: bool) -> i32 {
    let mut result = 0;
    text.split("\n").for_each(|line| {
        // eg line := "2-4,6,8"

        let mut segs = Vec::<(i32, i32)>::new();
        line.split(",").for_each(|range| {
            let seg: Vec<_> = range
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            segs.push((seg[0], seg[1]));
        });
        println!("segs: {:?}", segs);
        // segs := [(2,4), (6,8)]
        if fully_contains(segs[0], segs[1]) || fully_contains(segs[1], segs[0]) {
            result += 1;
        }
    });

    result
}

fn get_char_at(line: &str, offset: usize) -> char {
    line.chars().nth(offset).unwrap()
}

fn parse_col(text: &str, offset: usize, verbose: bool) -> Vec<char> {
    let mut result = Vec::<char>::new();
    for line in text.split("\n") {
        let c = line.chars().nth(offset).unwrap();
        result.push(c);
    }
    if verbose {
        println!("{:?}", result);
    }
    result
}

fn parse_col_to_string(text: &str, offset: usize, verbose: bool) -> String {
    parse_col(text, offset, verbose)
        .into_iter()
        .collect::<String>()
}

fn is_match(word: &str) -> bool {
    (word == "XMAS") || (word == "SAMX")
}

fn count_horiz(text: &str, verbose: bool) -> i32 {
    let mut result = 0;

    let lines: Vec<_> = text
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    if verbose {
        println!("{:?}", lines);
    }
    lines.iter().for_each(|line| {
        let windows = line.windows(4);
        windows.for_each(|w| {
            let word = w.into_iter().collect::<String>();
            //println!("word: {:?}", word);

            if is_match(&word) {
                result += 1;
            }
        });
    });

    result
}
fn count_vert(text: &str, verbose: bool) -> i32 {
    let mut result = 0;

    let mut lines = Vec::<Vec<char>>::new();
    for line in text.split("\n") {
        let mut row = Vec::<char>::new();
        for c in line.chars() {
            if c != '\n' {
                row.push(c);
            }
        }
        if !row.is_empty() {
            lines.push(row);
        }
    }
    println!("{:?}", lines);

    let num_cols = lines.get(0).unwrap().len();
    let num_rows = lines.len();

    for col_idx in 0..num_cols {
        let mut col = vec![];
        for row_idx in 0..num_rows {
            col.push(lines.get(row_idx).unwrap().get(col_idx).unwrap());
        }
        let windows = col.windows(4);
        windows.for_each(|w| {
            let word = w.into_iter().map(|c| c.to_string()).collect::<String>();
            if verbose {
                println!("word: {:?}", word);
            }

            if is_match(&word) {
                result += 1;
            }
        });
        //println!("col: {} {:?}", col_idx, col);
    }

    result
}
fn count_diag(text: &str, verbose: bool) -> i32 {
    let mut result = 0;

    let mut lines = Vec::<Vec<char>>::new();
    for line in text.split("\n") {
        let mut row = Vec::<char>::new();
        for c in line.chars() {
            if c != '\n' {
                row.push(c);
            }
        }
        if !row.is_empty() {
            lines.push(row);
        }
    }
    println!("{:?}", lines);

    let num_cols = lines.get(0).unwrap().len();
    let num_rows = lines.len();
    println!("cols: {}, rows {}", num_cols, num_rows);

    for col_idx in 0..(num_cols - 3) {
        for row_idx in 0..(num_rows - 3) {
            let mut v = vec![];
            for i in 0..4 {
                v.push(lines.get(row_idx + i).unwrap().get(col_idx + i).unwrap());
            }
            let word = v.into_iter().map(|c| c.to_string()).collect::<String>();
            if is_match(&word) {
                println!("1 diag ({}, {}) = {}", row_idx, col_idx, word);
                result += 1;
            }
            println!("word at {}:{} -> {}", row_idx, col_idx, word);
            //if (row_idx == 6) && (col_idx == 0) {
            //   println!("word at 0:6 -> {}", word);
            //}
        }
    }
    for col_idx in 3..num_cols {
        for row_idx in 0..(num_rows - 3) {
            let mut v = vec![];
            for i in 0..4 {
                v.push(lines.get(row_idx + i).unwrap().get(col_idx - i).unwrap());
            }
            let word = v.into_iter().map(|c| c.to_string()).collect::<String>();
            if is_match(&word) {
                println!("2 diag ({}, {}) = {}", row_idx, col_idx, word);
                result += 1;
            }
        }
    }

    result
}

fn part1(inp: &str, verbose: bool) -> i32 {
    let mut result = 0;

    result += count_horiz(inp, verbose);
    result += count_vert(inp, verbose);
    result += count_diag(inp, verbose);

    if verbose {
        println!("result: {:?}", result);
    }
    result
}
fn part2(inp: &str, verbose: bool) -> i32 {
    let mut result = 0;

    let mut lines = Vec::<Vec<char>>::new();
    for line in inp.split("\n") {
        let mut row = Vec::<char>::new();
        for c in line.chars() {
            if c != '\n' {
                row.push(c);
            }
        }
        if !row.is_empty() {
            lines.push(row);
        }
    }
    //println!("{:?}", lines);
    let num_cols = lines.get(0).unwrap().len();
    let num_rows = lines.len();
    println!("cols: {}, rows {}", num_cols, num_rows);
    for row_idx in 0..(num_rows - 2) {
        for col_idx in 0..(num_cols - 2) {
            let mut v = vec![];
            v.push(lines.get(row_idx).unwrap().get(col_idx).unwrap());
            v.push(lines.get(row_idx).unwrap().get(col_idx + 2).unwrap());

            v.push(lines.get(row_idx + 1).unwrap().get(col_idx + 1).unwrap());

            v.push(lines.get(row_idx + 2).unwrap().get(col_idx).unwrap());
            v.push(lines.get(row_idx + 2).unwrap().get(col_idx + 2).unwrap());
            let word = v.into_iter().map(|c| c.to_string()).collect::<String>();
            println!("({}, {}) = {}", row_idx, col_idx, word);

            if (word == "MMASS") || (word == "SSAMM") || (word == "MSAMS") || (word == "SMASM") {
                //println!("hit: {}, {}", row_idx, col_idx);
                result += 1;
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

    //let res1 = part1(&inp, false);
    //println!("result from part1: {}", res1);

    const TEXT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let res2 = part2(&inp, false);
    //let res2 = part2(&TEXT.to_string(), true);
    println!("result from part2: {}", res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEXT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(18, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(9, part2(&text, true));
    }
    //#[test]
    //fn test_part2() {
    //    let text = TEXT.to_string();
    //    assert_eq!(31, part2(&text, true));
    //}
}
