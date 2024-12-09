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

pub fn parse_map<T:Copy>(lines:Lines<BufReader<File>>, chars:HashMap<char, T>) -> Vec<Vec<T>> {
    let mut cells:Vec<Vec<T>> = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let mut cell_line = Vec::new();
        for char in line.chars() {
            if let Some(value) = chars.get(&char) {
                cell_line.push(*value);
            } else {
                panic!("Unknown character ");
            }
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
