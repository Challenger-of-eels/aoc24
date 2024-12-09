use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

pub fn main() {
    let input:ParseResult = parse(&mut common::get_input_lines(file!()));
    common::measure(p1, &input);
    common::measure(p2, &input);
}

#[test]
fn test_input_p1() {
    p1(&parse(&mut common::get_test_input_lines(file!())));
}

#[test]
fn test_input_p2() {
    p2(&parse(&mut common::get_test_input_lines(file!())));
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

fn parse<T>(lines:&mut T)->ParseResult where T: Iterator<Item = String> {
    let mut result = 0_i64;
    for l in lines {
        let line = l.unwrap();
    }
    Ok(result)
}