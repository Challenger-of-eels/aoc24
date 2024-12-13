use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;
use regex::Regex;

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


type ParseResult = Vec<Machine>;

#[derive(Debug, Clone, Copy)]
struct Machine {
    a:(i64,i64),
    b:(i64,i64),
    r:(i64,i64),
}

// m.a.0 * a + m.b.0 * b = m.r.0;
// m.a.1 * a + m.b.1 * b = m.r.1;
// ->
// b = (m.r.1 - m.a.1 * a) / m.b.1;
// m.a.0 * a + m.b.0 * ((m.r.1 - m.a.1 * a) / m.b.1) = m.r.0;
// ->
// a = (m.r.0 - m.b.0 * m.r.1 / m.b.1) / (m.a.0 - m.b.0 * m.a.1 / m.b.1); // * m.b.1
// ->
// a = (m.r.0 * m.b.1 - m.b.0 * m.r.1) / (m.a.0 * m.b.1 - m.b.0 * m.a.1);

fn p1(input:&ParseResult) {
    let mut result:i64 = 0;

    for m in input {

        let a_1 = m.r.0 * m.b.1 - m.b.0 * m.r.1;
        let a_2 = m.a.0 * m.b.1 - m.b.0 * m.a.1;

        if a_1 % a_2 == 0 {
            let a = a_1 / a_2;
            if (m.r.1 - m.a.1 * a) % m.b.1 == 0 {
                let b = (m.r.1 - m.a.1 * a) / m.b.1;
                result += 3 * a + b;
            }
        }
    }

    dbg!(result);
}

fn p2(input:&ParseResult) {
    let mut modified_machines = input.to_vec();
    for machine in modified_machines.iter_mut() {
        machine.r.0 += 10000000000000;
        machine.r.1 += 10000000000000;
    }
    p1(&modified_machines);
}

fn parse<T>(lines:&mut T)->ParseResult where T: Iterator<Item = String> {
    // Button A: X+43, Y+88
    // Button B: X+85, Y+39
    // Prize: X=5333, Y=5246
    fn parse_coords(s:&String) -> (i64,i64) {
        let reg = Regex::new(r": X[+=](\d+), Y[+=](\d+)").unwrap();
        for (_, [a, b]) in reg.captures_iter(s).map(|c| c.extract()) {
            return (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
        }
        panic!("No valid coords");
    }
    let mut machines = vec![];
    while let Some(l) = lines.next() {
        machines.push(Machine {
            a: parse_coords(&l),
            b: parse_coords(&lines.next().unwrap()),
            r: parse_coords(&lines.next().unwrap()),
        });
        lines.next(); // try skip new line
    }
    machines
}