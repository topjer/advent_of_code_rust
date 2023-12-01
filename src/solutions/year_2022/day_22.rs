#![allow(dead_code, unused_variables)]
use crate::solutions::read_file;

#[derive(PartialEq, Debug)]
enum Command {
    Steps(u32),
    Right,
    Left
}

pub fn solve() {
    let lines = read_file("./src/inputs/year_2022/day_22_unit");
    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
/* 
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {:?}", result_part_2) ;
    println!("------------");
    */
}

fn find_starting_position(first_row: &String) -> (u32, u32) {
    (0, first_row.find('.').unwrap() as u32)
}

fn parse_input(input: &Vec<String>) -> (Vec<&String>, Vec<Command>) {
    let mut map: Vec<&String> = Vec::new();
    let mut command_string: String = String::new();
    let mut map_mode: bool = true;
    for line in input {
        if line.len() == 0 {
            map_mode = false;
            continue;
        }

        if map_mode == true {
            map.push(line);
        } else {
            command_string = line.clone();
        }
    }
    let command = parse_command(command_string);
    (map, command)
}

fn logic_part_1(input: &Vec<String>) -> u32 {
    let (map, command) = parse_input(input);
    let position: (u32, u32) = find_starting_position(&map[0]);
    println!("first line: {:?}", position);
    let direction = (0, 1);
    for next_step in command {
        match next_step {
            Command::Steps(number_steps) => {
                
            },
            Command::Left => {

            },
            Command::Right => {

            }
        }
    }
    /* 
    let line: &String = &map[0];
    println!("Specific character {:?}", line.chars().nth(4).unwrap());
    let index = line.find('.').unwrap();
    println!("index position {:?}, {:?}", index, line.chars().nth(index).unwrap());
*/
    return 1;
}

fn turn_left(direction: (i32, i32)) -> (i32, i32) {
    ((-1)*direction.1, direction.0)
}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, (-1)*direction.0)
}

#[test]
fn test_turn_left() {
    assert!(turn_left((1,0)) == (0,1));
    assert!(turn_left((0,1)) == (-1,0));
    assert!(turn_left((-1,0)) == (0,-1));
    assert!(turn_left((0,-1)) == (1,0));
}

#[test]
fn test_turn_right() {
    assert!(turn_right((1,0)) == (0,-1));
    assert!(turn_right((0,1)) == (1,0));
    assert!(turn_right((-1,0)) == (0,1));
    assert!(turn_right((0,-1)) == (-1,0));
}

fn parse_command(cmd: String) -> Vec<Command> {
    let mut result: Vec<Command> = Vec::new();
    let mut number_string: String = "".to_string();
    for character in cmd.chars() {
        if char::is_numeric(character) {
            number_string.push(character)
        } else {
            result.push(Command::Steps(number_string.parse().unwrap()));
            if character == 'L' {
                result.push(Command::Left);
            } else {
                result.push(Command::Right)
            }
            number_string.clear();
        }
    }
    return result
}

#[test]
fn test_parse_command() {
    let result = parse_command(String::from("10R10L"));
    //println!("{:#?}", result);
    let expected = vec![Command::Steps(10),Command::Right,Command::Steps(10),Command::Left];
    for (item1, item2) in result.iter().zip(expected.iter()) {
        assert!(item1 == item2)
    }
}