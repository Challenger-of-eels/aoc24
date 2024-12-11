use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader, Lines}, ops::Index, path::Path};
use common::{self, discrete2d::{Direction, Map2d, V2}};

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

type ParseResult = Map2d<Cell>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Wall,
    Start,
}

fn p1(input:&ParseResult) {
    let map = input;
    let mut pos = V2::from_usize(map.find(Cell::Start).unwrap());

    let mut direction = Direction::Up;

    let mut visited:HashSet<(i32, i32)> = HashSet::new();
    visited.insert((pos.x, pos.y));

    loop {
        let mut new_pos = pos.clone();
        new_pos.step_in_direction(&direction, 1);
        if !map.contains(new_pos) {
            break;
        }
        if *map.get(new_pos) == Cell::Wall {
            direction = direction.rotate_cw();
            continue;
        }
        pos = new_pos;
        visited.insert((pos.x, pos.y));
    }

    dbg!(visited.len());

    // let chars = HashMap::from([
    //     ('.', Cell::Empty),
    //     ('#', Cell::Wall),
    //     ('^', Cell::Start),
    // ]);
    // let chars_inverted: HashMap<Cell, char> = chars.iter()
    // .map(|(k, v)| (*v, *k)).collect();
    // common::print_map(&map.cells, chars_inverted);
}

/*
 Brute force over all cells is a viable option,
 but its more efficient to check only blocks on current path
 1.7s
*/
fn p2(input:&ParseResult) {
    let map = input;
    let mut pos = V2::from_usize(map.find(Cell::Start).unwrap());
    let mut visited:HashSet<(i32, i32, Direction)> = HashSet::new();
    let mut tried:HashSet<(i32, i32)> = HashSet::new();
    let mut direction = Direction::Up;
    visited.insert((pos.x, pos.y, direction));

    fn is_loop(mut pos:V2, mut direction:Direction, early_visited:&HashSet<(i32, i32, Direction)>, map:&Map2d<Cell>, block:&(i32, i32)) -> bool {
        let mut visited:HashSet<(i32, i32, Direction)> = HashSet::new();
        visited.insert((pos.x, pos.y, direction));
        let mut step = 0;
        loop {
            let mut new_pos = pos.clone();
            new_pos.step_in_direction(&direction, 1);
            if !map.contains(new_pos) {
                break;
            }
            if (*map.get(new_pos) == Cell::Wall) || new_pos.equal_xy(block) {
                direction = direction.rotate_cw();
                continue;
            }
            pos = new_pos;
            if visited.contains(&(pos.x, pos.y, direction)) || early_visited.contains(&(pos.x, pos.y, direction)) {
                return true;
            }
            visited.insert((pos.x, pos.y, direction));
        }
        return false;
    }

    let mut result:i64 = 0;

    loop {
        let mut new_pos = pos.clone();
        new_pos.step_in_direction(&direction, 1);
        if !map.contains(new_pos) {
            break;
        }
        if (*map.get(new_pos) == Cell::Wall) {
            direction = direction.rotate_cw();
            continue;
        }
        let new_pos_pair = new_pos.to_i32();
        if !tried.contains(&new_pos_pair) {
            if is_loop(pos, direction, &visited, map, &new_pos_pair) {
                result += 1;
            }
            tried.insert(new_pos_pair);
        }
        pos = new_pos;
        visited.insert((pos.x, pos.y, direction));
    }


    dbg!(result);
}

fn parse(file:&Path)->Result<ParseResult,anyhow::Error> {
    let lines = BufReader::new(File::open(file)?).lines().into_iter();
    
    let map: Map2d<Cell> = Map2d::<Cell> {
        cells: common::parse_map(lines.map(|s| s.unwrap()), |c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            '^' => Cell::Start,
            _ => panic!("Not expected character")
        }
            
            // HashMap::from([
            // ('.', Cell::Empty),
            // ('#', Cell::Wall),
            // ('^', Cell::Start),
        // ])
        )
    };
    Ok(map)
}