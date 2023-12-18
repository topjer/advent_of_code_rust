use num::traits::float;
use std::i64;

use crate::solutions::read_file;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_18");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

fn parse_input2(input: &Vec<String>) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();
    input.iter().map(|l| l.split(' ').collect::<Vec<&str>>())
        .for_each(|c| {
            let mut d: Direction= Direction::Down;
            //println!("{:?}", c);
            let dir = c[2].chars().nth_back(1).unwrap().to_string().parse::<u32>().unwrap();
            //println!("{:?}", dir);
            let length: String = c[2].chars().skip(2).take(5).collect();
            match dir {
                0 => {d=Direction::Right},
                2 => {d=Direction::Left},
                1 => {d=Direction::Down},
                3 => {d=Direction::Up},
                _ => {println!("unknown direction")}
            }
            output.push(Instruction{
                direction: d,
                length: i64::from_str_radix(&length, 16).unwrap() as usize,
            })
        });
    output
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
            })
        });
    //println!("{:?}", output);
    output
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    row: isize,
    column: isize
}

struct Dimensions {
    max_row: isize,
    min_row: isize,
    max_col: isize,
    min_col: isize
}
fn find_endpoint(pos: &Point, dir: &Direction, length: &usize) -> Point {
    match dir {
        Direction::Up => {
            return Point{row: pos.row - *length as isize, column: pos.column};
        },
        Direction::Down => {
            return Point{row: pos.row + *length as isize, column: pos.column};
        },
        Direction::Left => {
            return Point{row: pos.row, column: pos.column - *length as isize};
        },
        Direction::Right => {
            return Point{row: pos.row, column: pos.column + *length as isize};
        },
    }
}

fn find_points(pos: &Point, dir: &Direction, length: &usize) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();
    match dir {
        Direction::Up => {
            for l in 1..=*length {
                output.push(Point{row: pos.row - l as isize, column: pos.column})
            }
        },
        Direction::Down => {
            for l in 1..=*length {
                output.push(Point{row: pos.row + l as isize, column: pos.column})
            }
        },
        Direction::Left => {
            for l in 1..=*length {
                output.push(Point{row: pos.row, column: pos.column - l as isize})
            }
        },
        Direction::Right => {
            for l in 1..=*length {
                output.push(Point{row: pos.row, column: pos.column + l as isize})
            }
        },
    }
    output
}

fn draw_map(map: &HashMap<Point, Direction>, dim: &Dimensions) {
    let mut output: String = "".to_string();
    for r in dim.min_row..=dim.max_row {
        for c in dim.min_col..=dim.max_col {
            if map.contains_key(&Point{row: r, column: c}) {
                //print!("#");
                output.push('#');
            } else {
                //print!(".");
                output.push('.');
            }
        }
        //println!();
        output.push('\n');
    }
    fs::write("./outputs/day_18", output).expect("Unable to write file");
}

fn count_points(map: &HashSet<Point>, dim: &Dimensions) -> u32 {
    let mut inner_points: Vec<Point> = Vec::new();
    for r in dim.min_row..=dim.max_row {
        let mut streak: usize = 0;
        let mut inside: bool = false;
        for c in dim.min_col..=dim.max_col {
            let p = Point{row: r, column: c};
            if map.contains(&p) {
                inner_points.push(p);
                if streak == 1 && map.contains(&Point{row: r-1, column: c-1}) {
                    inside = !inside;
                }
                streak += 1;
            } else {
                if streak == 1 {
                    inside = !inside;
                } 
                if streak > 1 && map.contains(&Point{row: r-1, column: c-1}) {
                    inside = !inside;
                }
                if inside == true {
                    inner_points.push(p);
                }
                streak = 0;
            }

        }
        //println!();
    }
    let mut output: String = "".to_string();
    for r in dim.min_row..=dim.max_row {
        for c in dim.min_col..=dim.max_col {
            if inner_points.contains(&Point{row: r, column: c}) {
                //print!("#");
                output.push('#');
            } else {
                //print!(".");
                output.push('.');
            }
        }
        output.push('\n');
    }
    fs::write("./outputs/day_18_filled", output).expect("Unable to write file");
    //inner_points.len();
    1
}

fn logic_part_1 (input: &Vec<String>) -> f32 {
    // shoelace formula + Picks formula
    let instructions = parse_input(input);
    let mut current_position: Point= Point{row: 0,column: 0};
    let mut area: f32 = 0.0;
    let mut cirumference: usize = 0;
    for instruction in instructions {
        let next_point = find_endpoint(&current_position, &instruction.direction, &instruction.length);
        area += 0.5 * (current_position.column + next_point.column) as f32 * (current_position.row - next_point.row) as f32;
        cirumference += instruction.length;
        current_position = next_point;
    }
    println!("Area: {}", area);
    println!("cirumference: {}", cirumference);
    let outcome = area.abs() + cirumference as f32 / 2.0 + 1.0;
    println!("outcome: {}", outcome);
    outcome
} 

fn logic_part_2 (input: &Vec<String>) -> f64 {
    let instructions = parse_input2(input);
    println!("{:?}", instructions);
    let mut current_position: Point= Point{row: 0,column: 0};
    let mut area: f64 = 0.0;
    let mut cirumference: f64 = 0.0;
    for instruction in instructions {
        let next_point = find_endpoint(&current_position, &instruction.direction, &instruction.length);
        area += 0.5 * (current_position.column + next_point.column) as f64 * (current_position.row - next_point.row) as f64;
        cirumference += instruction.length as f64;
        current_position = next_point;
        println!("{:?}", current_position);
    }
    println!("Area: {}", area.abs());
    println!("cirumference: {}", cirumference as f64 / 2.0);
    let outcome = area.abs() + cirumference as f64 / 2.0 + 1.0;
    println!("outcome: {}", outcome);
    outcome
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_18_unit");
    let result = logic_part_1(&lines);
    assert!(result == 62.0);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_18_unit");
    let result = logic_part_2(&lines);
    assert!(result == 952408144115.0);
}

#[test]
fn dict_test() {
    let mut foo: HashMap<usize, Vec<usize>> = HashMap::new();
    let temp: Vec<usize> = Vec::new();
    foo.insert(1, vec![10,20,30]);
    foo.get_mut(&1).unwrap().pop().unwrap();
    foo.get_mut(&1).unwrap().push(40);
    foo.get_mut(&2).get_or_insert(&mut temp.clone()).push(50);
    foo.entry(2).and_modify(|el| el.push(50)).or_insert(vec![50]);
    println!("{:?}", foo);
}