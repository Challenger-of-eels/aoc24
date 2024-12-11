use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

pub fn main() {
    let input:ParseResult = parse(&mut common::get_input_lines(file!()));
    common::measure(p1, &input); // 0.010525s
    common::measure(p2, &input); // 0.084806s
}

#[test]
fn test_input_p1() {
    p1(&parse(&mut common::get_test_input_lines(file!())));
}

#[test]
fn test_input_p2() {
    p2(&parse(&mut common::get_test_input_lines(file!())));
}

type ParseResult = Vec<u64>;

fn get_digits_count(num:u64)->u64 {
    (num as f64).log10() as u64 + 1
}

fn split_digits(num:u64, low_digits_count:u64)->(u64,u64) {
    let high_multiplier = 10_u64.pow(low_digits_count as u32);
    let high = num / high_multiplier;
    let low = num - high * high_multiplier;
    return (high, low);
}

fn next_generation(vec:&Vec<u64>) -> Vec<u64> {
    let mut new_vec = Vec::new();
    for &num in vec {
        let digits_count = get_digits_count(num);
        if num == 0 {
            new_vec.push(1);
        } else if digits_count % 2 == 0 {
            let split = split_digits(num, digits_count / 2);
            new_vec.push(split.0);
            new_vec.push(split.1);
        } else {
            new_vec.push(num * 2024);
        }
    }
    return new_vec;
}

fn p1(input:&ParseResult) {
    let mut vec:Vec<u64> = input.to_vec();
    for _ in 0..25 {
        vec = next_generation(&vec);
    }
    dbg!(vec.len());
}

fn add(map:&mut HashMap<u64, u64>, key:u64, mut value:u64) {
    if let Some (v) = map.get(&key) {
        value += *v;
    }
    map.insert(key, value);
}

fn next_generation_map(map:HashMap::<u64, u64>)->HashMap::<u64, u64> {
    let mut new_map = HashMap::new();
    for (num, count) in map {
        let digits_count = get_digits_count(num);
        if num == 0 {
            add(&mut new_map, 1, count);
        } else if digits_count % 2 == 0 {
            let split = split_digits(num, digits_count / 2);
            add(&mut new_map, split.0, count);
            add(&mut new_map, split.1, count);
        } else {
            add(&mut new_map, num * 2024, count);
        }
    }
    new_map
}

// checked that all one-digit nubmers falls back to one-digit numbers in a few iterations
// checked that if we can skip all one-digit numbers, bigger numbers are calculated pretty fast
// made assumption that a lot of numbers will fall into themselves one way or another multiple times
// so let's store every generation in a map, not in a vec
fn p2(input:&ParseResult) {
    let mut map:HashMap<u64, u64> = HashMap::from_iter(input.iter().map(|v| (*v, 1)));

    for _ in 0..75 {
        map = next_generation_map(map);
    }

    // dbg!(map.len()); // 3771 values in my example
    dbg!(map.values().sum::<u64>());
}

fn parse<T>(lines:&mut T)->ParseResult where T: Iterator<Item = String> {
    lines.next().unwrap().split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
}