use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

type ParseResult = Vec<Vec<char>>;

pub fn main() {
    let input:ParseResult = common::get_input(parse, file!());
    
    common::measure(p1, &input);
    common::measure(p2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_p1() {
        p1(&common::get_test_input(parse, file!()));
    }

    #[test]
    fn test_input_p2() {
        p2(&common::get_test_input(parse, file!()));
    }
}

fn has_palindrome(grid:&Vec<Vec<char>>, start:(i32, i32), step:(i32, i32), word:&str) -> bool {
    return has_word(grid, start, step, word)
        || has_word(
            grid,
            (start.0 + step.0 * (word.len() as i32 - 1), start.1 + step.1 * (word.len() as i32 - 1)),
            (-step.0, -step.1),
            word
        );
}

fn has_word(grid:&Vec<Vec<char>>, start:(i32, i32), step:(i32, i32), word:&str) -> bool {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut xy = start;
    for c in word.chars() {
        if xy.0 < 0 || xy.1 < 0 || xy.0 >= w || xy.1 >= h {
            return false;
        }
        if grid[xy.0 as usize][xy.1 as usize] != c {
            return false;
        }
        xy.0 += step.0;
        xy.1 += step.1;
    }
    return true;
}

fn p1(input:&ParseResult) {
    let mut result = 0;
    let mut steps = Vec::<(i32,i32)>::new();
    for x in [-1,0,1] {
        for y in [-1,0,1] {
            if !(x == 0 && y == 0) {
                steps.push((x, y));
            }
        }
    }
    let word= "XMAS";
    let h = input.len();
    let w = input[0].len();
    for y in 0..h {
        for x in 0..w {
            let start = (x as i32,y as i32);
            for step in &steps {
                if has_word(input, start, *step, word) {
                    result += 1;
                }
            }
        }
    }
    dbg!(result);
}

fn p2(input:&ParseResult) {
    let mut result = 0;
    let word= "MAS";
    let h = input.len();
    let w = input[0].len();
    for y in 1..(h as i32) - 1 {
        for x in 1..(w as i32) - 1 {
            if has_palindrome(input, (x - 1, y - 1), (1, 1), word) && has_palindrome(input, (x - 1, y + 1), (1, -1), word) {
                result += 1;
            }
        }
    }
    dbg!(result);
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = Vec::new();

    for l in lines {
        let line = l.unwrap();
        result.push(line.chars().collect());
    }
    Ok(result)
}