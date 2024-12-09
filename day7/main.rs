use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

pub fn main() {
    let input:ParseResult = parse(common::get_input_lines(file!()));
    
    common::measure(p1, &input);
    common::measure(p2, &input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_p1() {
        p1(&parse(common::get_test_input_lines( file!())));
    }

    #[test]
    fn test_input_p2() {
        p2(&parse(common::get_test_input_lines(file!())));
    }
}

type ParseResult = Vec<Task>;

#[derive(Debug, Eq, PartialEq)]
struct Task {
    result:i64,
    values:Vec<i64>,
}

// number of values is tiny so recursion is king

fn p1(input:&ParseResult) {
    fn try_solve(task: &Task, i:usize, value:i64) -> bool {
        if i == task.values.len() { return value == task.result; }
        return
            try_solve(task, i + 1, value + task.values[i]) ||
            try_solve(task, i + 1, value * task.values[i]);
    }

    let result:i64 = input.iter()
        .filter(|task| try_solve(task, 1, task.values[0]))
        .map(|task| task.result)
        .sum();
    dbg!(result);

}

fn p2(input:&ParseResult) {
    fn concat(a:i64, b:i64) -> i64 {
        let zeroes = 10_i64.pow((b as f64).log10() as u32 + 1);
        return a * zeroes + b;
    }

    fn try_solve(task: &Task, i:usize, value:i64) -> bool {
        if i == task.values.len() { return value == task.result; }
        return
            try_solve(task, i + 1, value + task.values[i]) ||
            try_solve(task, i + 1, value * task.values[i]) ||
            try_solve(task, i + 1, concat(value , task.values[i]));
    }
    
    let result:i64 = input.iter()
        .filter(|task| try_solve(task, 1, task.values[0]))
        .map(|task| task.result)
        .sum();
    dbg!(result);
}

fn parse<T>(lines:T)->ParseResult where T: Iterator<Item = String> {
    let mut result = Vec::new();
    for line in lines {
        let (result_str, tail_str) = line.split_once(": ").unwrap();
        let task = Task {
            result: result_str.parse::<i64>().unwrap(),
            values: tail_str.split(" ").map(|s| s.parse::<i64>().unwrap()).collect(),
        };
        result.push(task);
    }
    return result;
}