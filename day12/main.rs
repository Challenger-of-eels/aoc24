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

type ParseResult = Map2d<char>;

fn in_same_area(map:&Map2d<char>, pos:&(i32, i32), new_pos:&(i32, i32))->bool {
    return map.contains_i32_pair(&new_pos) && map.get_i32_pair(&new_pos) == map.get_i32_pair(&pos);
}

fn add(a:&(i32,i32), b:&(i32,i32))->(i32,i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn count_angles(map:&Map2d<char>, pos:&(i32,i32))->i64 {
    let mut angles = 0;
    // iterate over consecutive pairs of orthogonal directions, that form an angle
    for i in 0..4 {
        let step_a = DIRECTIONS[i].get_step();
        let step_b = DIRECTIONS[(i + 1) % 4].get_step();
        let step_diagonal = add(&step_a, &step_a);

        let in_a= in_same_area(map, &pos, &add(&pos, &step_a));
        let in_b= in_same_area(map, &pos, &add(&pos, &step_b));
        let in_diagonal= in_same_area(map, &pos, &add(&pos, &step_diagonal));
        
        let concave_angle = !in_a && !in_b;
        let convex_angle = in_a && in_b && !in_diagonal;
        if concave_angle || convex_angle {
            angles += 1;
        }
    }
    return angles
}

fn evaluate(map:&Map2d<char>, start:(i32,i32), visited:&mut HashSet<(i32,i32)>)->(i64, i64, i64) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut angles = 0;
    let mut stack:Vec::<(i32, i32)> = vec!(start);
    visited.insert(start);
    while let Some(pos) = stack.pop() {
        area += 1;

        for dir in DIRECTIONS {
            let step = dir.get_step();
            let new_pos = add(&pos, &step);
            if in_same_area(&map, &pos, &new_pos) {
                if !visited.contains(&new_pos) {
                    visited.insert(new_pos);
                    stack.push(new_pos);
                }
            } else {
                perimeter += 1;
            }
        }

        angles += count_angles(&map, &pos);
    }
    return (area, perimeter, angles);
}

fn p1(input:&ParseResult) {
    let mut result:i64 = 0;
    let mut visited:HashSet<(i32,i32)> = HashSet::new();
    for x in 0..input.width() {
        for y in 0..input.height() {
            let start = (x as i32, y as i32);
            if visited.contains(&start) { continue; }
            let (area, perimeter, angles) = evaluate(input, start, &mut visited);
            result += area * perimeter;
        }
    }
    dbg!(result);
}

fn p2(input:&ParseResult) {
    let mut result:i64 = 0;
    let mut visited:HashSet<(i32,i32)> = HashSet::new();
    for x in 0..input.width() {
        for y in 0..input.height() {
            let start = (x as i32, y as i32);
            if visited.contains(&start) { continue; }
            
            let (area, perimeter, angles) = evaluate(input, start, &mut visited);
            result += area * angles;
        }
    }
    dbg!(result);
}

fn parse<T>(lines:&mut T)->ParseResult where T: Iterator<Item = String> {
    Map2d::<char> {
        cells: common::parse_map(lines, |c| c)
    }
}