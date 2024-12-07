use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

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

//#[derive(Debug, Clone, Copy, Eq, PartialEq)]

type ParseResult = i64;

fn p1(input:&ParseResult) {
    let mut result:i64 = 0;
    dbg!(input);
}

fn p2(input:&ParseResult) {
    // let mut result:i64 = 0;
    // dbg!(result);
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = 0_i64;

    for l in lines {
        let line = l.unwrap();
    }
    Ok(result)
}