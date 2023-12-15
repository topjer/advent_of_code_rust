use crate::solutions::read_file;
use std::{collections::HashMap, clone};

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_15");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>) -> Vec<&str> {
    input[0].split(',').collect()
}

fn logic_part_1 (input: &Vec<String>) -> usize {
    let steps = parse_input(input);
    println!("{:?}", steps);
    let res: usize = steps.iter().map(|s| hash_function(s)).sum();
    res
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let steps = parse_input(input);
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    for step in steps {
        if step.ends_with('-') {
            let label = &step[0..step.len()-1];
            let box_id = hash_function(label);
            if boxes.contains_key(&box_id) {
                let mut box_content = boxes.get(&box_id).unwrap().clone();
                if let Some(pos) = box_content.iter().position(|(x,y)| *x == label) {
                    box_content.remove(pos);
                }
                *boxes.get_mut(&box_id).unwrap() = box_content;
            }
            println!("Remove stuff");

        } else {
            let parts: Vec<&str> = step.split('=').collect();
            let addition: (&str, usize) = (parts[0], parts[1].parse().unwrap());
            let box_id = hash_function(parts[0]);
            if boxes.contains_key(&box_id) {
                let mut vec_addition = boxes.get(&box_id).unwrap().clone();
                let label_pos = vec_addition.iter().position(|(x,y)| *x == parts[0]);
                if let Some(pos) = label_pos {
                    println!("label {:?} present", parts[0]);
                    vec_addition[pos] = addition;
                } else {
                    vec_addition.push(addition);
                }
                *boxes.get_mut(&box_id).unwrap() = vec_addition.clone();
                print!("");
            } else {
                let temp = vec![addition];
                boxes.insert(box_id, temp);
            }
        }
    }
    let mut focussing_power = 0;
    for (b_id, lens_box) in boxes.iter() {
        for (slot, lens) in lens_box.iter().enumerate() {
            focussing_power += (b_id + 1) * (slot + 1) * lens.1;
        }

    }
    //println!("{:?}", boxes);
    focussing_power
}

fn hash_function(input: &str) -> usize {
    let mut current_value: usize= 0;
    for c in input.chars() {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256; 
    }
    current_value
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_15_unit");
    let result = logic_part_1(&lines);
    assert!(result == 1320);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_15_unit");
    let result = logic_part_2(&lines);
    assert!(result == 145);
}

#[test]
fn hash_test() {
    let input = "HASH";
    assert!(hash_function(input) == 52 as usize);
}