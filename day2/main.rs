use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

type ParseResult = Vec<Vec<i64>>;

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

fn p1(input:&ParseResult) {
    let mut result:i64 = 0;
    for a in input {
        if is_valid(a) {
            result += 1;
        }
    }
    dbg!(result);
}

fn is_valid(a:&Vec<i64>)->bool {
    let mut direction:i64 = 0;
    for i in 1..a.len() {
        let prev:i64 = a[i - 1];
        let item:i64 = a[i];

        if (item == prev) || ((item - prev).abs() > 3_i64) {
            return false;
        }

        let this_direction:i64 = if item > prev { 1_i64 } else if item < prev { -1_i64 } else { 0 };
        if direction == 0 {
            direction = this_direction;
        } else if direction != this_direction {
            return false;
        }
    }
    return true;
}

fn p2(input:&ParseResult) {
    let mut result:i64 = 0;
    for a in input {
        if is_valid_damped(a) {
            result += 1;
        }
    }
    dbg!(result);
}

fn is_valid_damped(a:&Vec<i64>)->bool {
    let error_index = is_valid_skip(a, a.len() + 1);
    if error_index == 0 {
        return true;
    }
    if is_valid_skip(a, (error_index - 1) as usize) == 0 || is_valid_skip(a, error_index) == 0 {
        return true;
    }
    if error_index > 1 && is_valid_skip(a, (error_index - 2) as usize) == 0 {
        return true;
    }
    return false;
}

fn is_valid_skip(a:&Vec<i64>, skip_index:usize)->usize {
    let mut direction:i64 = 0;
    let undefined_prev = -1_i64;
    let mut prev:i64 = undefined_prev;
    for i in 0..a.len() {
        if i == skip_index {
            continue;
        }
        let item:i64 = a[i];
        if prev == undefined_prev {
            prev = item;
            continue;
        }

        if (item == prev) || ((item - prev).abs() > 3_i64) {
            return i;
        }

        let this_direction:i64 = if item > prev { 1_i64 } else if item < prev { -1_i64 } else { 0 };
        if direction == 0 {
            direction = this_direction;
        } else if direction != this_direction {
            return i;
        }
        prev = item;
    }
    if (skip_index != a.len() + 1) {
        dbg!(skip_index, a);
    }
    return 0;
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let a:Vec<i64> = line.split(" ").map(|str| str.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        result.push(a);
    }
    Ok(result)
}