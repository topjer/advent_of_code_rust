#![allow(dead_code)]

use crate::solutions::read_file;
use std::cmp::max;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_02");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {:?}", result_part_2) ;
    println!("------------");
}

/// Iterate over all input lines first split at ':'. This will separate the 'Game #' part from the 
/// outcomes. Then split at ';' to get the actual pulls in that game.
/// 
/// Next, go through all pulls and split at ',' to get the individual sets of cubes being used.
/// 
/// Lastly, split at ' ' to separate the number from the color. Note that the strings start with a
/// space thus we ignore the first entry when matching.
/// 
/// The actual logic is to check the numbers against the known thresholds and if they are exceeded then
/// we go to the next game. (We labeled the outer for loop for that).
/// 
/// Only if all thresholds are obeyed will we get to the part where we add the id of the game to the
/// overall sum. Remember that indices start at 0 but games start at 1.
/// 
fn logic_part_1(input: &Vec<String>) -> u32 {
    let mut sum: usize = 0;
    'outer: for (id, line) in input.iter().enumerate() {
        let parts: Vec<&str> = line.split(':').collect();
        let games: Vec<&str> = parts[1].split(';').collect();
        for game in games {
            let cubes: Vec<&str> = game.split(',').collect();
            for cube in cubes {
                let config: Vec<&str> = cube.split(' ').collect();
                //println!("{:?}", config);
                match config[..] {
                   [_, number, "blue"] => {if number.parse::<u32>().unwrap() > 14 as u32 {continue 'outer;}}
                   [_, number, "red"] => {if number.parse::<u32>().unwrap() > 12 as u32 {continue 'outer;}},
                   [_, number, "green"] =>{if number.parse::<u32>().unwrap() > 13 as u32 {continue 'outer;}},
                   _ => print!("none")
                }
            }
        }
        //println!("will add {:?}", id);
        sum += id + 1;
    }
    sum as u32
}

/// Similar parsing here, the main difference is that we determine the maximum number for each color
/// for a given game. Thus, with every new game we have to reset the values for red, green, blue.
/// 
fn logic_part_2(input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let parts = line.split(':');
        let games = parts.into_iter().skip(1).next().unwrap().split(';');
        let (mut red, mut green, mut blue): (u32, u32, u32) = (0,0,0);
        for game in games {
            let cubes = game.split(',');
            for cube in cubes {
                let config: Vec<&str> = cube.split(' ').collect();
                println!("{:?}", config);
                match config[..] {
                   [_, number, "blue"] => {blue = max(blue, number.parse::<u32>().unwrap())}
                   [_, number, "red"] => {red = max(red, number.parse::<u32>().unwrap())},
                   [_, number, "green"] =>{green = max(green, number.parse::<u32>().unwrap())},
                   _ => print!("none")
                }
            }
        }
        println!("Could have been played with: {} {} {}", red, green, blue);
        sum+= red * green * blue;
    }
    sum
}


#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_02_unit1");
    let result = logic_part_1(&lines);
    assert!(result == 8);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_02_unit1");
    let result = logic_part_2(&lines);
    assert!(result == 2286);
}