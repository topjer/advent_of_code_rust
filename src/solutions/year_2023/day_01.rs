#![allow(dead_code)]

use std::collections::HashMap;
use core::num;

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

fn logic_part_1(input: &Vec<String>) -> u32 {
    let mut numeric_vec: Vec<u32> = Vec::new();
    for line in input {
        let mut holder: Vec<char> = Vec::new();
        let mut number: Vec<char> = Vec::new();
        for pos in line.chars() {
            if char::is_numeric(pos) {
                holder.push(pos);
            }
        }
        //println!("Numerics found: {:?}", holder);
        number.push(holder[0]);
        number.push(holder.pop().unwrap());
        let number_from_string: u32 = number.iter().collect::<String>().parse().unwrap();
        //println!("Numbers found: {:?}", number_from_string);
        numeric_vec.push(number_from_string);
    }
    let result = numeric_vec.iter().sum();
    result
}

fn logic_part_2(input: &Vec<String>) -> u32 {
    let string_dict: HashMap<String, u32> = define_dict();
    let mut result: u32 = 0;
    for line in input {
        let mut all_matches: Vec<(usize, &str)> = Vec::new();
        for key in string_dict.keys() {
            //let mut holder: Vec<(u32, String)> = Vec::new();
            let matches: Vec<(usize, &str)> = line.match_indices(key).collect();
            all_matches.extend(&matches);
        }
        all_matches.sort_by_key(|k| k.0);
        //println!("Matches found: {:?}", all_matches);
        let a = string_dict.get(all_matches[0].1).unwrap();
        let b = string_dict.get(all_matches.pop().unwrap().1).unwrap();
        //println!("number: {}", 10 * a + b);
        result += 10 * a + b;
    }
    result
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_01_unit");
    let result = logic_part_1(&lines);
    assert!(result == 142);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_01_unit2");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}

fn define_dict() -> HashMap<String, u32> {
    let mut string_dict: HashMap<String, u32> = HashMap::new();
    string_dict.insert("1".to_string(), 1);
    string_dict.insert("2".to_string(), 2);
    string_dict.insert("3".to_string(), 3);
    string_dict.insert("4".to_string(), 4);
    string_dict.insert("5".to_string(), 5);
    string_dict.insert("6".to_string(), 6);
    string_dict.insert("7".to_string(), 7);
    string_dict.insert("8".to_string(), 8);
    string_dict.insert("9".to_string(), 9);
    string_dict.insert("0".to_string(), 0);
    string_dict.insert("one".to_string(), 1);
    string_dict.insert("two".to_string(), 2);
    string_dict.insert("three".to_string(), 3);
    string_dict.insert("four".to_string(), 4);
    string_dict.insert("five".to_string(), 5);
    string_dict.insert("six".to_string(), 6);
    string_dict.insert("seven".to_string(), 7);
    string_dict.insert("eight".to_string(), 8);
    string_dict.insert("nine".to_string(), 9);
    string_dict.insert("zero".to_string(), 0);
    string_dict
}