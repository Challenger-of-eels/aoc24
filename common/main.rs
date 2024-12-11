use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}, path::Path, time::Instant};

pub fn measure<T>(p:fn(input:&T), input:&T) {
    let now = Instant::now();
    p(input);
    let t = now.elapsed().as_micros() as f64 / 1e6;
    println!("Elapsed {}s", t);
}



pub fn get_input_lines(invoker_path:&str) -> impl Iterator<Item = String> {
    let file = Path::new(invoker_path).join("../input.txt");
    let lines = BufReader::new(File::open(file).unwrap()).lines();
    return lines.map(|s| s.unwrap());
}

pub fn get_test_input_lines(invoker_path:&str) -> impl Iterator<Item = String> {
    let file = Path::new("../").join(invoker_path).join("../input.test.txt");
    let lines = BufReader::new(File::open(file).unwrap()).lines();
    return lines.map(|s| s.unwrap());
}

pub fn get_input<T>(parser:fn(file:&Path)->Result<T,anyhow::Error>, invoker_path:&str)->T {
    let file = Path::new(invoker_path).join("../input.txt");
    return parser(&file).unwrap();
}


pub fn get_test_input<T>(parser:fn(file:&Path)->Result<T,anyhow::Error>, invoker_path:&str)->T {
    let file = Path::new("../").join(invoker_path).join("../input.test.txt");
    return parser(&file).unwrap();
}

pub fn parse_map<T:Copy, StringIterator>(lines:StringIterator, chars:fn(char)->T) -> Vec<Vec<T>>
where StringIterator: Iterator<Item = String> {
    let mut cells:Vec<Vec<T>> = Vec::new();
    for line in lines {
        let mut cell_line = Vec::new();
        for char in line.chars() {
            cell_line.push(chars(char));
        }
        cells.push(cell_line);
    }
    return cells;
}


pub fn print_map<T:Copy + std::hash::Hash + Eq>(cells:&Vec<Vec<T>>, chars:HashMap<T, char>) {
    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            print!("{}", chars.get(&cells[y][x]).unwrap());
        }
        println!();
    }
}


pub fn iterate_all_pairs<'a: 'c, 'b: 'c, 'c, T>(
    xs: &'a [T],
    ys: &'b [T],
) -> impl Iterator<Item = (&'a T, &'b T)> + 'c {
    xs.iter().flat_map(move |x| std::iter::repeat(x).zip(ys))
}


pub fn iterate_all_unique_pairs<'a: 'c, 'b: 'c, 'c, T>(
    xs: &'a [T],
    ys: &'b [T],
) -> impl Iterator<Item = (&'a T, &'b T)> + 'c where T:PartialEq {
    return xs.iter().flat_map(move |x| std::iter::repeat(x).zip(ys)).filter(|(a, b)| *a != *b);
}


pub mod discrete2d {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct V2 {
        pub x:i32,
        pub y:i32,
    }
    
    impl V2 {
        pub fn new() -> V2 {
            return V2 { x:0, y:0 };
        }
    
        pub fn to_usize(&self) -> (usize, usize) {
            return (self.x as usize, self.y as usize);
        }
    
        pub fn to_i32(&self) -> (i32, i32) {
            return (self.x, self.y);
        }
    
        pub fn from_usize(coords:(usize, usize)) -> V2 {
            return V2 { x:coords.0 as i32, y:coords.1 as i32 };
        }
    
        pub fn from_i32(coords:(i32, i32)) -> V2 {
            return V2 { x:coords.0, y:coords.1 };
        }
    
        pub fn equal_xy(&mut self, check:&(i32, i32)) -> bool {
            return self.x == check.0 && self.y == check.1;
        }
    
        pub fn add_xy(&mut self, addition:(i32, i32)) {
            self.x += addition.0;
            self.y += addition.1;
        }
    
        pub fn add(&mut self, v:V2) {
            self.x += v.x;
            self.y += v.y;
        }
    
        pub fn step_in_direction(&mut self, direction:&Direction, step:i32) {
            match direction {
                Direction::Right => self.x += step,
                Direction::Left => self.x -= step,
                Direction::Down => self.y += step,
                Direction::Up => self.y -= step,
            }
        }
    }
    
    #[derive(Debug)]
    pub struct Map2d<T> {
        pub cells:Vec<Vec<T>>,
    }
    
    impl<T:Eq> Map2d<T> {
        pub fn width(&self) -> usize {
            return self.cells[0].len();
        }
    
        pub fn height(&self) -> usize {
            return self.cells.len();
        }
    
        pub fn contains(&self, pos:V2) -> bool {
            return pos.x >= 0 && pos.x < self.width() as i32 && pos.y >= 0 && pos.y < self.height() as i32;
        }

        pub fn contains_i32_pair(&self, pair:&(i32, i32)) -> bool {
            return pair.0 >= 0 && pair.0 < self.width() as i32 && pair.1 >= 0 && pair.1 < self.height() as i32;
        }
    
        pub fn get(&self, pos:V2) -> &T {
            return self.get_i32_xy(pos.x, pos.y);
        }
    
        pub fn get_i32_xy(&self, x:i32, y:i32) -> &T {
            return self.get_xy(x as usize, y as usize);
        }
    
        pub fn get_i32_pair(&self, pair:&(i32, i32)) -> &T {
            return self.get_xy(pair.0 as usize, pair.1 as usize);
        }
    
        pub fn get_xy(&self, x:usize, y:usize) -> &T {
            return &self.cells[y][x];
        }
    
        pub fn find(&self, target:T) -> Option<(usize,usize)> {
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
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    pub static DIRECTIONS:[Direction;4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
    
    impl Direction {
        pub fn rotate_cw(&self) -> Direction {
            return match &self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        }
    
        pub fn get_step(&self) -> (i32, i32) {
            return match &self {
                Direction::Right => (1, 0),
                Direction::Left => (-1, 0),
                Direction::Down => (0, 1),
                Direction::Up => (0, -1),
            }
        }
    }
}