use crate::solutions::read_file;
use std::collections::HashMap;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: usize,
    color: String
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();
    input.iter().map(|l| l.split(' ').collect::<Vec<&str>>())
        .for_each(|c| {
            let mut d: Direction= Direction::Down;
            match c[0] {
                "R" => {d=Direction::Right},
                "L" => {d=Direction::Left},
                "D" => {d=Direction::Down},
                "U" => {d=Direction::Up},
                _ => {println!("unknown direction")}
            }
            output.push(Instruction{
                direction: d,
                length: c[1].parse::<usize>().unwrap(),
                color: c[2].to_string()
            })
        });
    println!("{:?}", output);
    output
}

struct Point {
    row: isize,
    column: isize
}

fn find_points(pos: Point, dir: Direction, length: usize) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();
    match dir {
        Direction::Up => {
            for l in 1..=length {
                output.push(Point{row: pos.row - l as isize, column: pos.column})
            }
        },
        Direction::Down => {
            for l in 1..=length {
                output.push(Point{row: pos.row + l as isize, column: pos.column})
            }
        },
        Direction::Left => {
            for l in 1..=length {
                output.push(Point{row: pos.row, column: pos.column - l as isize})
            }
        },
        Direction::Right => {
            for l in 1..=length {
                output.push(Point{row: pos.row, column: pos.column + l as isize})
            }
        },
    }
    output
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let instructions = parse_input(input);
    let current_position: Point= Point{row: 0,column: 0};
    let dug_out: HashMap<Point, usize> = HashMap::new();
    for instruction in instructions {

    }
    1
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    1
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_18_unit");
    let result = logic_part_1(&lines);
    assert!(result == 142);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day__unit");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}