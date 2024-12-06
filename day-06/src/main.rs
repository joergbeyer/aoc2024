#![allow(dead_code)]
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Default, Debug, PartialEq)]
struct Matrix {
    elements: Vec<Vec<char>>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_idx in 0..self.elements.len() {
            let row = self.elements.get(row_idx).unwrap();
            for c in row.iter() {
                _ = write!(f, "{}", c);
            }
            _ = write!(f, "\n");
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Matrix {
    fn new(s: &str) -> Matrix {
        let mut cols = Vec::<Vec<char>>::new();
        for line in s.split("\n") {
            if !line.is_empty() {
                let row: Vec<char> = line.chars().collect();
                cols.push(row);
            }
        }

        Matrix { elements: cols }
    }

    fn get(&self, row_idx: usize, col_idx: usize) -> &char {
        self.elements.get(row_idx).unwrap().get(col_idx).unwrap()
    }
    fn get_seq(&self, seq: &[(usize, usize)]) -> String {
        let mut result = String::new();
        for &(row_idx, col_idx) in seq {
            let row: &Vec<char> = self.elements.get(row_idx).unwrap();
            let c: char = *row.get(col_idx).unwrap();
            result.push(c);
        }
        result
    }

    fn find_first(&self, wanted: char) -> Option<(usize, usize)> {
        for row_idx in 0..self.elements.len() {
            let row = self.elements.get(row_idx).unwrap();
            for col_idx in 0..row.len() {
                if row.get(col_idx).unwrap() == &wanted {
                    return Some((row_idx, col_idx));
                }
            }
        }
        None
    }
    fn count(&self, wanted: char) -> usize {
        let mut result = 0;

        for row_idx in 0..self.elements.len() {
            let row = self.elements.get(row_idx).unwrap();
            for col_idx in 0..row.len() {
                if row.get(col_idx).unwrap() == &wanted {
                    result += 1;
                }
            }
        }
        result
    }
    fn size(&self) -> usize {
        let mut result = 0;

        for row_idx in 0..self.elements.len() {
            let row = self.elements.get(row_idx).unwrap();
            result += row.len();
        }
        result
    }
    fn update(&mut self, row_idx: usize, col_idx: usize, val: char) {
        *self
            .elements
            .get_mut(row_idx)
            .unwrap()
            .get_mut(col_idx)
            .unwrap() = val;
    }

    fn move_pos(&mut self, pos: &mut (usize, usize), dir: &mut Direction) -> bool {
        // returns true if still on map
        let mut row_idx = pos.0;
        let mut col_idx = pos.1;
        match dir {
            Direction::Up => {
                if row_idx > 0 {
                    if *self.get(row_idx - 1, col_idx) != '#' {
                        self.update(row_idx, col_idx, 'X');
                        row_idx -= 1;
                        self.update(row_idx, col_idx, '^');
                    } else {
                        *dir = Direction::Right;
                    }
                } else {
                    return false;
                }
            }
            Direction::Down => {
                if row_idx < self.elements.len() - 1 {
                    if *self.get(row_idx + 1, col_idx) != '#' {
                        self.update(row_idx, col_idx, 'X');
                        row_idx += 1;
                        self.update(row_idx, col_idx, 'v');
                    } else {
                        *dir = Direction::Left;
                    }
                } else {
                    return false;
                }
            }
            Direction::Left => {
                //let row = self.elements.get_mut(row_idx);
                if col_idx > 0 {
                    if *self.get(row_idx, col_idx - 1) != '#' {
                        self.update(row_idx, col_idx, 'X');
                        col_idx -= 1;
                        self.update(row_idx, col_idx, '<');
                    } else {
                        *dir = Direction::Up;
                    }
                } else {
                    return false;
                }
            }
            Direction::Right => {
                let row = self.elements.get_mut(row_idx).unwrap();
                if col_idx < row.len() - 1 {
                    if *self.get(row_idx, col_idx + 1) != '#' {
                        self.update(row_idx, col_idx, 'X');
                        col_idx += 1;
                        self.update(row_idx, col_idx, '>');
                    } else {
                        *dir = Direction::Down;
                    }
                } else {
                    return false;
                }
            }
        }

        pos.0 = row_idx;
        pos.1 = col_idx;
        true
    }
    //fn get_row(&self)
}
#[cfg(test)]
mod matrix_tests {
    use super::*;
    const SMALL: &str = "ABC
DEF";
    const MEDIUM: &str = "ABCDEFGH
IJKLMNOP
QRSTUVWX
abcdefgh
ijklmnop
qrstuvwx";
    #[test]
    fn test_ctr() {
        let m = Matrix::new(SMALL);
        assert_eq!(m.get(0, 0), &'A');
        assert_eq!(m.get(0, 1), &'B');
    }
    #[test]
    fn test_seq() {
        let m = Matrix::new(SMALL);
        let seq = vec![(0, 0), (0, 1), (1, 0)];
        let res = m.get_seq(&seq);
        assert_eq!("ABD", res);
    }
    #[test]
    fn test_find_first() {
        let m = Matrix::new(MEDIUM);
        assert_eq!(Some((2, 3)), m.find_first('T'));
    }

    #[test]
    fn test_update() {
        let mut m = Matrix::new(MEDIUM);
        assert_eq!(Some((2, 3)), m.find_first('T'));
        m.update(2, 3, '#');
        assert_eq!(Some((2, 3)), m.find_first('#'));
    }
}

fn part1(inp: &str, verbose: bool) -> usize {
    let mut map = Matrix::new(inp);

    let Some(mut pos) = map.find_first('^') else {
        todo!()
    };
    dbg!(&pos);
    let mut dir = Direction::Up;
    while map.move_pos(&mut pos, &mut dir) {}

    println!("{}", map);
    let result = map.count('X') + 1; // +1, because the last position is still ^,v,<,>
    if verbose {
        println!("result: {:?}", result);
    }
    result
}

fn is_endless(mut map: Matrix) -> bool {
    let Some(mut pos) = map.find_first('^') else {
        todo!()
    };
    let mut dir = Direction::Up;
    let map_size = map.size();
    //dbg!(&map_size);

    let mut cnt = 0;
    while map.move_pos(&mut pos, &mut dir) {
        cnt += 1;
        if cnt > map_size {
            return true;
        }
    }

    false
}

fn part2(inp: &str) -> usize {
    let mut result = 0;

    let v: Vec<_> = inp.chars().collect();
    for (i, el) in v.clone().into_iter().enumerate() {
        if el == '.' {
            let (left, right) = v.split_at(i);
            let mut tmp: Vec<char> = left.to_vec().clone();
            tmp.push('#');
            tmp.extend_from_slice(&right[1..]);

            let s = String::from_iter(tmp);
            let map = Matrix::new(&s);
            if is_endless(map) {
                result += 1
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = fs::read_to_string("input1.txt")?;

    //let res1 = part1(&inp, false);
    //println!("result from part1: {}", res1);

    let res2 = part2(&inp);
    println!("result from part2: {}", res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    const TEXT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let text = TEXT.to_string();
        assert_eq!(41, part1(&text, true));
    }
    #[test]
    fn test_part2() {
        let text = TEXT.to_string();
        assert_eq!(6, part2(&text, true));
    }
}
