#![allow(unused_assignments)]
use crate::solutions::read_file;
use std::{collections::HashMap, process::Output};

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_05");

    println!("Day 5");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let mut result: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut input_iter = input.iter();
    let seeds: Vec<u32> = input_iter
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut holder: Vec<Vec<u32>>= Vec::new();
    for line in input_iter {
        if line.ends_with(':') {continue;}
        if line.is_empty() {
            if !holder.is_empty() {
                result.push(holder);
            }
            holder = Vec::new()
        } else {
            holder.push(
                line.split(' ').map(|x| x.parse::<u32>().unwrap()).collect()
            )   
        }
    }
    // get the last section not covered by loop
    result.push(holder);
    //println!("{:?}", result);
    (seeds, result)
}

fn map_vector(map: Vec<Vec<u32>>, input: Vec<u32>) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();
        'outer: for entry in input {
            for range in &map {
                let diff: i64 = entry as i64 - range[1] as i64;
                if diff < 0 { continue; }
                if diff < range[2] as i64 {
                    let new = range[0] + diff as u32;
                    output.push(new);
                    continue 'outer;
                }
            }
            output.push(entry);
        }
        output
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let (seeds, maps) = parse_input(input);
    let mut to_transform = seeds.clone();
    for map in maps {
        to_transform = map_vector(map, to_transform);
        println!("{:?}", to_transform);
    }
    *to_transform.iter().min().unwrap()
}

fn reverse_map(number: u32, map:&Vec<Vec<u32>>) -> u32 {
    let mut result: u32= number;
    for range in map {
        let diff: i64 = number as i64 - range[0] as i64;
        if diff < 0 {continue;}
        if diff < range[2] as i64 {
            result = range[1] + diff as u32;
            break;
        }
    }
    result
}

fn check_seeds(number: u32, seeds: &Vec<u32>) -> bool {
    let mut result:bool = false;
    for index in 0..seeds.len()/2 {
        let diff: i64 = number as i64 - seeds[2*index] as i64;
        if diff < 0 {continue;}
        if diff < seeds[(2*index)+1] as i64 {
            result = true; break;
        }
    }
    result
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    let (seeds, maps) = parse_input(input);
    let mut result = 0;
    for number in 0..1000000000 as u32{
        let mut mapped_number = number;
        for map in maps.iter().rev() {
            mapped_number = reverse_map(mapped_number, map);
        }
        //println!("real number: {}, mapped number: {}", number, mapped_number);
        let in_seeds = check_seeds(mapped_number, &seeds);
        if in_seeds == true {
            //println!("Found seed: {}", mapped_number);
            result = number;
            break;
        }
    }
    //println!("result: {}", result);
    result
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_05_unit1");
    let result = logic_part_1(&lines);
    assert!(result == 35);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_05_unit2");
    let result = logic_part_2(&lines);
    assert!(result == 53);
}

#[test]
fn test_example3_input() {
    let lines = read_file("./src/inputs/year_2023/day_05_unit1");
    let result = logic_part_2(&lines);
    assert!(result == 46);
}