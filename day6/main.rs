use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader, Lines}, ops::Index, path::Path};
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

type ParseResult = Map<Cell>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct V2 {
    x:i32,
    y:i32,
}

impl V2 {
    fn new() -> V2 {
        return V2 { x:0, y:0 };
    }

    fn to_usize(&self) -> (usize, usize) {
        return (self.x as usize, self.y as usize);
    }

    fn to_i32(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    fn from_usize(coords:(usize, usize)) -> V2 {
        return V2 { x:coords.0 as i32, y:coords.1 as i32 };
    }

    fn from_i32(coords:(i32, i32)) -> V2 {
        return V2 { x:coords.0, y:coords.1 };
    }

    fn equal_xy(&mut self, check:(i32, i32)) {
        return self.x == chech.x && self.y == check.y;
    }

    fn add_xy(&mut self, addition:(i32, i32)) {
        self.x += addition.0;
        self.y += addition.1;
    }


    fn add(&mut self, v:V2) {
        self.x += v.x;
        self.y += v.y;
    }

    fn step_in_direction(&mut self, direction:&Direction, step:i32) {
        match direction {
            Direction::Right => self.x += step,
            Direction::Left => self.x -= step,
            Direction::Down => self.y += step,
            Direction::Up => self.y -= step,
        }
    }
}

#[derive(Debug)]
struct Map<T> {
    cells:Vec<Vec<T>>,
}

impl<T:Eq> Map<T> {
    fn width(&self) -> usize {
        return self.cells[0].len();
    }

    fn height(&self) -> usize {
        return self.cells.len();
    }

    fn contains(&self, pos:V2) -> bool {
        return pos.x >= 0 && pos.x < self.width() as i32 && pos.y >= 0 && pos.y < self.height() as i32;
    }

    fn get(&self, pos:V2) -> &T {
        return self.get_int_xy(pos.x, pos.y);
    }

    fn get_int_xy(&self, x:i32, y:i32) -> &T {
        return self.get_xy(x as usize, y as usize);
    }

    fn get_xy(&self, x:usize, y:usize) -> &T {
        return &self.cells[y][x];
    }

    fn find(&self, target:T) -> Option<(usize,usize)> {
        for y in 0..self.cells.len() {
            for x in 0..self.width() {
                if self.cells[y][x] == target {
                    return Some((x, y));
                }
            }
        }
        return None;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Wall,
    Start,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        return match &self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_step(&self) -> (i32, i32) {
        return match &self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        }
    }
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


    let mut result:i64 = 0;
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

    fn is_loop(mut pos:V2, mut direction:Direction, early_visited:&HashSet<(i32, i32, Direction)>, map:&Map<Cell>, block:&(i32, i32)) -> bool {
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

    let map:Map<Cell> = Map::<Cell> {
        cells: common::parse_map(lines, HashMap::from([
            ('.', Cell::Empty),
            ('#', Cell::Wall),
            ('^', Cell::Start),
        ]))
    };
    Ok(map)
}