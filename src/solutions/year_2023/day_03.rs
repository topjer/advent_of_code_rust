#![allow(dead_code, unused_mut)]
use regex::Regex;
use crate::solutions::read_file;
use core::num;
use std::collections::HashMap;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_03");

    println!("Day 3");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {:?}", result_part_2) ;
    println!("------------");
}

#[derive(Debug)]
struct NumberPosition {
    value: usize,
    line_number: usize,
    start: usize,
    end: usize
}

struct Symbol {
    symbol: String,
    counter: u32
}

fn parse_input(input: &Vec<String>) -> (Vec<NumberPosition>, HashMap<(usize, usize), &str>, Vec<(usize, usize)>) {
    let mut numbers_vec: Vec<NumberPosition> = Vec::new();
    let mut symbol_lookup: HashMap<(usize, usize), &str> = HashMap::new();
    let mut gear_vec: Vec<(usize, usize)> = Vec::new();
    for (line_index, line) in input.iter().enumerate() {
        for cap in Regex::new(r"\d+").unwrap().find_iter(line) {
            let number = cap.as_str().parse::<usize>().unwrap();
            numbers_vec.push(NumberPosition { value: number, line_number: line_index, start: cap.start(), end: cap.end() })
        }
        for cap in Regex::new(r"[^\.\d]").unwrap().find_iter(line) {
            symbol_lookup.insert((line_index, cap.start()), cap.as_str());
        }
        for cap in Regex::new(r"\*").unwrap().find_iter(line) {
            gear_vec.push((line_index, cap.start()));
        }
    }
    (numbers_vec, symbol_lookup, gear_vec)
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let mut sum: usize = 0;
    let (numbers_vec, symbol_lookup, _) = parse_input(input);
    // check through lookups
    'outer: for number_pos in &numbers_vec {
        // check left and right of number
        if number_pos.start > 0 && symbol_lookup.contains_key(&(number_pos.line_number, number_pos.start - 1)) {
            sum += number_pos.value;
            continue;
        }
        if symbol_lookup.contains_key(&(number_pos.line_number, number_pos.end)) {
            sum += number_pos.value;
            continue;
        }
        //check line above
        for index_position in (number_pos.start as isize - 1).max(0) as usize..number_pos.end + 1 {
                if symbol_lookup.contains_key(&((number_pos.line_number as isize - 1).max(0) as usize, index_position)) {
                    sum += number_pos.value;
                    continue 'outer
                }
                if symbol_lookup.contains_key(&((number_pos.line_number + 1).max(0), index_position)) {
                    sum += number_pos.value;
                    continue 'outer
                }
        }
    }
    sum as u32
}

fn check_symbol(number_pos: &NumberPosition, gear_vec: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
    let mut result: Option<(usize, usize)> = None;
    for index_position in (number_pos.start as isize -1).max(0) as usize..number_pos.end + 1 {
        if gear_vec.contains(&((number_pos.line_number as isize - 1).max(0) as usize, index_position )) {
            result = Some(((number_pos.line_number as isize - 1).max(0) as usize, index_position ));
        }
        if gear_vec.contains(&(number_pos.line_number, index_position )) {
            result = Some((number_pos.line_number, index_position ));
        }
        if gear_vec.contains(&(number_pos.line_number + 1, index_position )) {
            result = Some((number_pos.line_number + 1, index_position ));
        }
    }
    result
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let mut sum: usize = 0;
    let mut holder: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let (numbers_vec,_, gear_vec) = parse_input(input);
    let mut sum: usize = 0;
    //println!("{:#?}", gear_vec);
    for number_pos in &numbers_vec {
        let found_symbol = check_symbol(number_pos, &gear_vec);
        match found_symbol {
            Some(point) => {
                if holder.contains_key(&point) {
                    holder.get_mut(&point).unwrap().push(number_pos.value);
                } else {
                    holder.insert(point, vec![number_pos.value]);
                }
            },
            None => continue
        }
    }
    for entry in holder {
        if entry.1.len() == 2 {
            sum += entry.1.iter().product::<usize>()
        }
    }
    sum
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_03_unit1");
    let result = logic_part_1(&lines);
    assert!(result == 4361);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_03_unit1");
    let result = logic_part_2(&lines);
    assert!(result == 467835);
}