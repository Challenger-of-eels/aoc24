use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;
use regex::Regex;

type ParseResult = i64;

pub fn main() {
    let input:ParseResult = common::get_input(parse, file!());
    let input2:ParseResult = common::get_input(parse2, file!());
    
    common::measure(p1, &input);
    common::measure(p2, &input2);
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
        p2(&common::get_test_input(parse2, file!()));
    }
}

fn p1(input:&ParseResult) {
    dbg!(input);
}

fn p2(input:&ParseResult) {
    dbg!(input);
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = 0_i64;

    for l in lines {
        let line = l.unwrap();

        // mul(X,Y)
        let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for (_, [a, b]) in reg.captures_iter(&line).map(|c| c.extract()) {
            result += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
        }
    }
    Ok(result)
}



fn parse2(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = 0_i64;

    let mut enabled = true;
    for l in lines {
        let line = l.unwrap();

        // mul(X,Y)
        // do()
        // don't()
        let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))()|(don't\(\))()").unwrap();
        for (_, [a, b]) in reg.captures_iter(&line).map(|c| c.extract()) {
            if a == "do()" {
                enabled = true;
            } else if a == "don't()" {
                enabled = false;
            } else if enabled {
                result += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
            }
        }
    }
    Ok(result)
}