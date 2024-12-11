use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path};
use common::{self, discrete2d::{Direction, Map2d, DIRECTIONS}};

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

type ParseResult = Map2d<i32>;

fn find_routes_count_from(pos:(i32, i32), map:&Map2d<i32>, unique_visited_summits:&mut Option<HashSet<(i32,i32)>>) -> i64 {
    if *map.get_i32_pair(&pos) == 9 {
        if let Some(summits) = unique_visited_summits {
            if summits.contains(&pos) {
                return 0;
            } else {
                summits.insert(pos);
            }
        }
        return 1;
    }

    let mut result = 0;
    for direction in DIRECTIONS {
        let step = direction.get_step();
        let new_pos = (pos.0 + step.0, pos.1 + step.1);
        if !map.contains_i32_pair(&new_pos) { continue; }
        if *map.get_i32_pair(&new_pos) == map.get_i32_pair(&pos) + 1 {
            result += find_routes_count_from(new_pos, map, unique_visited_summits);
        }
    }
    return result;
}

fn p1(input:&ParseResult) {
    let mut result:i64 = 0;
    for x in 0..input.width() as i32 {
        for y in 0..input.height() as i32 {
            if *input.get_i32_xy(x, y) == 0 {
                result += find_routes_count_from((x, y), &input, &mut Some(HashSet::new()));
            }
        }
    }
    dbg!(result);
}

fn p2(input:&ParseResult) {
    let mut result:i64 = 0;
    for x in 0..input.width() as i32 {
        for y in 0..input.height() as i32 {
            if *input.get_i32_xy(x, y) == 0 {
                result += find_routes_count_from((x, y), &input, &mut None);
            }
        }
    }
    dbg!(result);
}

fn parse<T>(lines:&mut T)->ParseResult where T: Iterator<Item = String> {
    Map2d::<i32> {
        cells: common::parse_map(lines, |c| c.to_digit(10).unwrap() as i32)
    }
}