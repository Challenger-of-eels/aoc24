use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, iter, ops::Index, path::Path};
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

type ParseResult = Task;

#[derive(Debug)]
struct Task {
    size:(i32,i32),
    nodes:HashMap<char, Vec<(i32,i32)>>,
}

fn in_size(size:(i32, i32), v:(i32, i32)) -> bool {
    return v.0 >= 0 && v.1 >= 0 && v.0 < size.0 && v.1 < size.1;
}

fn include_harmonics(unique:&mut HashSet<(i32, i32)>, size:(i32, i32), a:(i32, i32), b:(i32, i32), harmonics:i32) -> bool {
    let test_point = (
        b.0 + (b.0 - a.0) * harmonics,
        b.1 + (b.1 - a.1) * harmonics
    );
    if in_size(size, test_point) {
        if !unique.contains(&test_point) {
            unique.insert(test_point);
        }
        return true;
    }
    return false;
}

fn p1_healthy_man_solution(input:&ParseResult) {
    let mut unique_pairs= HashSet::new();
    for nodes in input.nodes.values() {
        for a in nodes {
            for b in nodes {
                if a == b { continue; }
                include_harmonics(&mut unique_pairs, input.size, *a, *b, 1);
            }
        }
    }
    dbg!(unique_pairs.len());
}

fn iterate_all_unique_pairs<'a: 'c, 'b: 'c, 'c, T>(
    xs: &'a [T],
    ys: &'b [T],
) -> impl Iterator<Item = (&'a T, &'b T)> + 'c where T:PartialEq {
    return xs.iter().flat_map(move |x| std::iter::repeat(x).zip(ys)).filter(|(a, b)| *a != *b);
}

// hacky way to replace 3 simple nested loops with spooky oneliner, for educational purposes
fn p1(input:&ParseResult) {
    let mut unique_pairs= HashSet::new();
    for (a, b) in input.nodes.values().flat_map(|v| iterate_all_unique_pairs(v, v)) {
        include_harmonics(&mut unique_pairs, input.size, *a, *b, 1);
    }
    dbg!(unique_pairs.len());
}

fn p2(input:&ParseResult) {
    let mut unique_pairs:HashSet<(i32, i32)> = HashSet::new();
    for (a, b) in input.nodes.values().flat_map(|v| iterate_all_unique_pairs(v, v)) {
        for harmonics in 0.. {
            let in_bounds = include_harmonics(&mut unique_pairs, input.size, *a, *b, harmonics);
            if !in_bounds { break; }
        }
    }
    dbg!(unique_pairs.len());
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines();
    let mut result = 0_i64;

    let mut nodes:HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for l in lines {
        let line = l.unwrap();
        x = 0;
        for c in line.chars() {
            if c != '.' {
                if let Some(node_vec) = nodes.get_mut(&c) {
                    node_vec.push((x, y));
                } else {
                    nodes.insert(c, Vec::from([(x, y)]));
                }
            };
            x += 1;
        }
        y += 1;
    }
    Ok(Task {
        size: (x, y),
        nodes,
    })
}