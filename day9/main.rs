use std::{cell::RefCell, collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, ops::Index, path::Path, u32};
use common;

pub fn main() {
    let input:ParseResult = parse(&mut common::get_input_lines(file!()));
    common::measure(p1, &input); // 0.000594s
    common::measure(p2_bruteforce, &input); // 0.155961s
    common::measure(p2_fast, &input); // 0.001029s
}

#[test]
fn test_input_p1() {
    p1(&parse(&mut common::get_test_input_lines(file!())));
}

#[test]
fn test_input_p2() {
    p2_bruteforce(&parse(&mut common::get_test_input_lines(file!())));
}

#[test]
fn test_input_p2_fast() {
    p2_fast(&parse(&mut common::get_test_input_lines(file!())));
}

type ParseResult = Vec<u32>;

fn get_file_check_sum(pos:u32, size:u32, id:u32) -> u64 {
    return (size * pos + size * (size - 1) / 2) as u64 * id as u64;
}

fn p1(input:&ParseResult) {
    let mut result:i64 = 0;

    let l:i32 = input.len() as i32;

    let mut id_left:i32 = 0;
    let mut id_right:i32 = (l + 1) / 2;

    let mut index_left:i32 = -1;
    let mut index_right:i32 = ((l - 1) / 2) * 2 + 2;

    let mut blocks_right = 0;
    let mut blocks_left = 0;

    let mut pos = 0;

    'full: loop {
        loop {
            if (index_left >= index_right) && (blocks_left == 0) {
                break 'full;
            }
            
            if blocks_right == 0 {
                index_right -= 2;
                if index_left >= index_right {
                    blocks_right = blocks_left;
                } else {
                    blocks_right = input[index_right as usize];
                }
                id_right -= 1;
                continue;
            }
    
            if blocks_left == 0 {
                index_left += 1;
                if index_left >= index_right {
                    blocks_left = blocks_right;
                } else {
                    blocks_left = input[index_left as usize];
                }
                id_left = index_left / 2;
                continue;
            }
            break;
        }

        let free_space_left = index_left % 2 == 1;
        if free_space_left || (id_left == id_right) {
            blocks_right -= 1;
            result += (id_right * pos) as i64;
        } else {
            result += (id_left * pos) as i64;
        }
        if blocks_left == 0 {
            dbg!(index_left, index_right);
        }
        blocks_left -= 1;
        pos += 1;
    }

    dbg!(result);
}

fn index_space(mem:&Vec<u32>) -> (Vec::<(u32,u32)>, Vec::<(u32,u32)>) {
    let mut empty = Vec::<(u32,u32)>::new();
    let mut files = Vec::<(u32,u32)>::new();

    let mut pos = 0;
    for i in 0..mem.len() {
        let size = mem[i];
        if i % 2 == 0 {
            files.push((pos, size));
        } else {
            empty.push((pos, size));
        }
        pos += size;
    }
    return (empty, files);
}

fn p2_bruteforce(input:&ParseResult) {
    let (mut empty, mut files) = index_space(input);

    let mut result = 0_u64;
    let mut id = files.len() as u32;
    
    for (file_pos, file_size) in files.iter_mut().rev() {
        id -= 1;
        for (empty_pos, empty_size) in empty.iter_mut().take(id as usize) {
            if file_size <= empty_size {
                *file_pos = *empty_pos;
                *empty_pos += *file_size;
                *empty_size -= *file_size;
                break;
            }
        }
        result += get_file_check_sum(*file_pos, *file_size, id);
    }
    dbg!(result);
}

// not 100% optimal but good enough
fn p2_fast(input:&ParseResult) {
    let (mut empty, mut files) = index_space(input);

    // we strictly have 10 digits as possible file sizes
    let mut min_empty:Vec<usize> = vec![0; 10];

    let mut result = 0_u64;
    let mut id = files.len() as u32;
    
    for (file_pos, file_size) in files.iter_mut().rev() {
        id -= 1;

        let size = *file_size as usize;
        for pos in min_empty[size]..id as usize {
            if empty[pos].1 >= *file_size {
                min_empty[size] = pos;

                let (empty_pos, empty_size) = &mut empty[pos];
                *file_pos = *empty_pos;
                *empty_pos += *file_size;
                *empty_size -= *file_size;
                break;
            }
        }
        
        result += get_file_check_sum(*file_pos, *file_size, id);
    }
    dbg!(result);
}

fn parse<T>(lines:&mut T)->ParseResult where T: Iterator<Item = String> {
    return lines.next().unwrap().chars().map(|s| s.to_digit(10).unwrap()).collect();
}