use std::{vec, ops::Index};
use std::collections::HashMap;

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_14");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>) -> Vec<String>{
    let mut holder: Vec<String> = Vec::new();
    for i in 0..input.last().unwrap().len() {
        holder.push(String::new());
    }
    for (line_id, line) in input.iter().enumerate() {
        for (c_id, c) in line.chars().enumerate() {
            holder[c_id].push(c)
        }
    }
    //println!("{:?}", holder);
    holder
}


fn logic_part_1 (input: &Vec<String>) -> u32 {
    let columns = parse_input(input);
    let mut total_load: usize = 0;
    for column in columns {
        println!("{column}");
        let col_length = column.len();
        let mut current_length = 0;
        let sections: Vec<&str>= column.split('#').collect();
        //let sections: Vec<&str>= column.split('#').filter(|el| el.len() > 0).collect();
        let old_total_load = total_load;
        for section in sections {
            let mut l: Vec<char> = section.chars().collect::<Vec<char>>();
            l.sort_unstable();
            println!("{l:?}");
            for (c_id, c) in l.into_iter().rev().enumerate() {
                if c == 'O' {
                    total_load += col_length - current_length - c_id;
                }
            }
            current_length += section.len() + 1;
        }
        println!("{}", total_load - old_total_load);
    }
    total_load as u32
}

fn move_in_line(line: &str) -> String {
    let mut temp = "".to_string();
    let mut empty_holder = "".to_string();
    for char in line.chars() {
        if char == 'O' {
            temp.push(char);
        } else if char == '.' {
            empty_holder.push(char);
        } else {
            temp.push_str(empty_holder.as_str());
            empty_holder.clear();
            temp.push(char)
        }
    }
    temp.push_str(empty_holder.as_str());
    temp
}

fn move_west(input: &Vec<String>) -> Vec<String> {
    let mut holder: Vec<String> = Vec::new();
    for i in 0..input[0].len() {
        let mut temp = "".to_string();
        for row in input {
            temp.push(row.chars().nth(i).unwrap());
        }
        holder.push(move_in_line(&temp));
    }
    holder
}

fn move_east(input: &Vec<String>) -> Vec<String> {
    let mut holder: Vec<String> = Vec::new();
    for i in 0..input[0].len() {
        let mut temp = "".to_string();
        for row in input.iter().rev() {
            temp.push(row.chars().nth(i).unwrap());
        }
        holder.push(move_in_line(&temp).chars().rev().collect());
    }
    holder
}

fn print_map(input: &Vec<String>) {
    for line in input {
        println!("{}",line);
    }
}

fn perform_cycle(input: &Vec<String>) -> Vec<String> {
    // names do not fit but moves happen to fit ...
    // did not investigate why XD
    let moved_north = move_west(&input);
    let moved_west = move_west(&moved_north);
    let moved_south = move_east(&moved_west);
    let moved_east = move_east(&moved_south);
    moved_east
}

fn calculate_load(input: &Vec<String>) -> usize {
    let mut load: usize = 0;
    for (id, line) in input.iter().enumerate() {
        load += (input.len() - id) * line.chars().map(|x| if x == 'O' {1} else {0}).sum::<usize>()
    }
    load
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    //let columns = parse_input(input);
    let mut temp = input.clone();
    let mut tables: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..200 {
        println!("Current cycle: {}", (i+1) % 52);
        temp = perform_cycle(&temp);
        let load = calculate_load(&temp);
        if tables.contains_key(&load) {
            tables.get_mut(&load).unwrap().push(i)
        } else {
            tables.insert(load, vec![i]);
        }
        println!("Calculated load: {load}");
    }
    println!("It has been previously determined that the cycle length is 52");
    println!("And 1000000000 % 52 = 12, so any index that is congruent 12 % 52 is the solution.");
    1

}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_14_unit");
    let result = logic_part_1(&lines);
    assert!(result == 136);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_14_unit");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}

#[test]
fn test_vector_joining() {
    println!("{}", 1000000000 % 52);
    println!("{}", 93 % 7);
}