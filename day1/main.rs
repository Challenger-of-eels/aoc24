use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

type ParseResult = (Vec<i64>, Vec<i64>);

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

fn p1(input:&(Vec<i64>,Vec<i64>)) {
    let mut a = input.0.clone();
    let mut b = input.1.clone();
    a.sort();
    b.sort();
    let mut result:i64 = 0;
    for (i, a_value) in a.iter().enumerate() {
        result += (b[i] - a_value).abs();
    }
    dbg!(result);
}

fn p2(input:&(Vec<i64>,Vec<i64>)) {
    let a = &input.0;
    let b = &input.1;
    let mut count_map:HashMap<i64, i64> = HashMap::new();
    for b_value in b.iter() {
        let count = count_map.get(b_value).unwrap_or(&0_i64) + 1_i64;
        count_map.insert(*b_value, count);
    }
    let mut result:i64 = 0;
    for a_value in a.iter() {
        result += a_value * count_map.get(a_value).unwrap_or(&0_i64);
    }
    dbg!(result);
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result:(Vec<i64>, Vec<i64>) = ([].to_vec(), [].to_vec());

    for l in lines {
        let line = l.unwrap();
        let (num1, num2) = line.split_once("   ").unwrap();
        
        result.0.push(num1.parse::<i64>().unwrap());
        result.1.push(num2.parse::<i64>().unwrap());
    }
    Ok(result)
}