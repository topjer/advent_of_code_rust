#![allow(unused_mut)]
use num::integer::lcm;
use std::collections::HashMap;
use regex::Regex;
use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_08");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>) -> (Vec<u32>, HashMap<String, Vec<String>>) {
    let mut line_iter = input.iter();
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    let instructions: Vec<u32> = line_iter.next().unwrap()
                                .chars()
                                .map(|c| (if c == 'L'{0} else {1}))
                                .collect();
    for line in line_iter.skip(1) {
        let cap_vec: Vec<String> = Regex::new(r"[A-Z|\d]{3}").unwrap()
                                .find_iter(line)
                                .map(|cap| cap.as_str().to_string())
                                .collect();
        network.insert(cap_vec[0].clone(), vec![cap_vec[1].clone(), cap_vec[2].clone()]);
    }
    //println!("{:?}", instructions);
    //println!("{:?}", network);
    (instructions, network)
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let (instructions, network) = parse_input(input);
    let mut counter: usize = 0;
    let mut position = "AAA";
    loop {
        let index = counter % instructions.len() as usize;
        let instruction = instructions[index] as usize;
        let choices = network.get(position).unwrap();
        position = &choices[instruction];
        if position == "ZZZ" {
            break;
        } else {
            counter += 1
        }
    }
    println!("{}", counter);
    counter as u32 + 1
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    let (instructions, network) = parse_input(input);
    let mut positions: Vec<String> = network.keys().filter(|key| key.ends_with('A')).map(|c| c.clone()).collect();
    println!("{:?}", positions);
    let mut cycle_lengths: Vec<u32> = Vec::new();
    for position in positions {
        let mut temp_pos = position.clone();
        let first_pos = position.clone();
        println!("Starting position: {}", position);
        for counter in (0..100000).into_iter() {
            let index = counter % instructions.len() as usize;
            let instruction = instructions[index] as usize;
            let choices = network.get(&temp_pos).unwrap();
            temp_pos = choices[instruction].clone();
            if temp_pos.ends_with('Z') {
                println!("Position {} at index {}", temp_pos, index);
                println!("{}", counter);
                cycle_lengths.push(counter as u32+ 1);
                break;
            }
        }

    }
        println!("{:?}", cycle_lengths);
    determine_result(cycle_lengths) as u32
    /* 
    loop {
        let mut holder = pos.clone();
        let max = holder.iter().max().unwrap();
        for (i, p) in holder.iter().enumerate() {
            if p < max {pos[i] += cycle_lengths[i]}
        }
        if pos.iter().all(|x| x == max) {break}
    }
    */
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_08_unit");
    let result = logic_part_1(&lines);
    assert!(result == 2);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_08_unit2");
    let result = logic_part_2(&lines);
    assert!(result == 6);
}

fn determine_result(cycle_lengths: Vec<u32>) -> u64 {
    let mut temp = cycle_lengths.clone().pop().unwrap() as u64;
    for element in cycle_lengths {
        println!("{}", temp);
        temp = lcm(temp, element as u64);
    }
    println!("Final result: {}", temp);
    temp
}