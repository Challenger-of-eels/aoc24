use std::{cell::RefCell, cmp::Ordering, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common;

#[derive(Debug)]
struct ParseResult {
    rules:Vec<(i32, i32)>,
    updates:Vec<Vec<i32>>,
}

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

fn build_rules_map(rules:&Vec<(i32, i32)>) -> HashMap<i32,HashSet<i32>> {
    let mut result = HashMap::<i32,HashSet::<i32>>::new();
    for &(a, b) in rules {
        
        if let Some(hash) = result.get_mut(&a) {
            hash.insert(b);
        } else {
            result.insert(a, HashSet::from([b]));
        }
    }
    return result;
}

fn update_is_valid(update:&Vec<i32>, rules:&HashMap<i32,HashSet<i32>>) -> bool {
    let l = update.len();
    for i in 1..l {
        if let Some(hash) = rules.get(&update[i]) {
            for j in 0..i {
                if hash.contains(&update[j]) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn p1(input:&ParseResult) {
    let mut result:i32 = 0;
    let rules = build_rules_map(&input.rules);
    for update in &input.updates {
        if update_is_valid(update, &rules) {
            result += update[update.len() / 2];
        }
    }
    dbg!(result);
}

fn p2(input:&ParseResult) {
    let mut result:i32 = 0;
    let rules = build_rules_map(&input.rules);

    fn compare(rules:&HashMap<i32,HashSet<i32>>, a:&i32, b:&i32)->Ordering {
        if let Some(hash) = rules.get(a) {
            if hash.contains(&b) {
                return Ordering::Greater;
            }
        }
        return Ordering::Less;
    }

    fn fix_update(rules:&HashMap<i32,HashSet<i32>>, update:Vec<i32>) -> Vec<i32> {
        let mut update = update;
        update.sort_by(|a, b| compare(rules, a, b));
        return update;
    }

    for update in &input.updates {
        if !update_is_valid(update, &rules) {
            let fixed_update = fix_update(&rules, update.to_vec());
            result += fixed_update[fixed_update.len() / 2];
        }
    }
    dbg!(result);
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = ParseResult {
        rules: Vec::<(i32, i32)>::new(),
        updates: Vec::<Vec<i32>>::new(),
    };

    let mut rules = true;
    for l in lines {
        let line = l.unwrap();
        if line.len() == 0 {
            rules = false;
            continue;
        }
        if rules {
            let (a_str,b_str) = line.split_once("|").unwrap();
            result.rules.push((a_str.parse::<i32>().unwrap(), b_str.parse::<i32>().unwrap()));
        } else {
            result.updates.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<_>() );
        }
    }
    Ok(result)
}