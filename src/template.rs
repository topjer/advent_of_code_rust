#![allow(dead_code)]

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_01");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {:?}", result_part_2) ;
    println!("------------");
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    1
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    1
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_02_unit1");
    let result = logic_part_1(&lines);
    assert!(result == 142);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_02_unit2");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}