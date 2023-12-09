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
    let mut cycle_start: Vec<usize>= Vec::new();
    let mut cycle_lengths: Vec<usize> = Vec::new();
    for position in &positions {
        let mut counter: usize = 0;
        let mut temp_pos = position.clone();
        let mut z_counter = 0;
        let mut last_z = 0;
        println!("Starting position: {}", position);
        loop {
            let index = counter % instructions.len() as usize;
            let instruction = instructions[index] as usize;
            let choices = network.get(&temp_pos).unwrap();
            temp_pos = choices[instruction].clone();
            if temp_pos.ends_with('Z') {
                z_counter += 1;
                if z_counter == 2 {
                    break
                    cycle_lengths.push(counter - last_z);
                } else {
                    println!("{:?}", counter - last_z);
                    last_z = counter;
                    cycle_start.push(counter)
                }
            } else {
                counter += 1
            }
        }

    }
        println!("{:?}", cycle_start);
        println!("{:?}", cycle_lengths);
        let mut pos = cycle_start;
    loop {
        let mut holder = pos.clone();
        let max = holder.iter().max().unwrap();
        for (i, p) in holder.iter().enumerate() {
            if p < max {pos[i] += cycle_lengths[i]}
        }
        if pos.iter().all(|x| x == max) {break}
    }
    println!("{:?}", pos);
    1
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

#[test]
fn foo() {
    let a = (13200, 2149);
    let b = (20568, 3684);
    let mut pos = vec![13200, 20568];
    let increments = vec![2149, 3684];
    let lcm_calc = lcm(a.1, b.1);
    println!("{}", a.0 % a.1);
    println!("{}", b.0 % b.1);
    println!("{}", lcm_calc);
    loop {
        let mut holder = pos.clone();
        let max = holder.iter().max().unwrap();
        for (i, p) in holder.iter().enumerate() {
            if p < max {pos[i] += increments[i]}
        }
        if pos.iter().all(|x| x == max) {break}
    }
    println!("{:?}", pos);
}