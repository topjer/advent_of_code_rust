use std::collections::HashMap;
use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_04");

    println!("Day 4");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for (line_index, line) in input.iter().enumerate() {
        let part: Vec<&str> = line.split(':').collect();
        let game_info: Vec<&str> = part[1].split('|').collect();
        let winning_numbers: Vec<&str> = game_info[0].split(' ').filter(|x| x.len() > 0).collect();
        let actual_numbers: Vec<&str> = game_info[1]
            .split(' ')
            .filter(|x| x.len() > 0)
            .filter(|x| winning_numbers.contains(x))
            .collect();
        if actual_numbers.len() > 0 {
            sum += (2 as u32).pow(actual_numbers.len() as u32 - 1);
        }
        /* 
        actual_numbers.iter()
            .map(|x| (2 as u32).pow(x.len() as u32-1));
        */
    }
    sum
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    let winning_numbers = determine_winning_numbers(input);
    //println!("{:?}", winning_numbers);
    let mut total_number: Vec<u32> = vec![1; winning_numbers.len()];
    //println!("{:?}", total_number);
    for (index, entry) in winning_numbers.iter().enumerate() {
        for number in 1..=*entry {
            let foo = index + number as usize;
            if foo < total_number.len() {
                total_number[foo] += total_number[index];
            }
        }
    }
    //println!("{:?}", total_number);
    total_number.iter().sum()
}

fn determine_winning_numbers (input: &Vec<String>) -> Vec<u32> {
    let mut winners: Vec<u32> = Vec::new();
    for (line_index, line) in input.iter().enumerate() {
        let part: Vec<&str> = line.split(':').collect();
        let game_info: Vec<&str> = part[1].split('|').collect();
        let winning_numbers: Vec<&str> = game_info[0].split(' ').filter(|x| x.len() > 0).collect();
        let actual_numbers = game_info[1]
            .split(' ')
            .filter(|x| x.len() > 0)
            .filter(|x| winning_numbers.contains(x))
            .count();
        winners.push(actual_numbers as u32);
    }
    winners 
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_04_unit1");
    let result = logic_part_1(&lines);
    assert!(result == 13);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_04_unit1");
    let result = logic_part_2(&lines);
    assert!(result == 30);
}