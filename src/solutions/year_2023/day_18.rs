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

fn count_points(map: &HashSet<Point>, dim: &Dimensions) -> usize {
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
    inner_points.len()
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let instructions = parse_input(input);
    let mut current_position: Point= Point{row: 0,column: 0};
    let mut dug_out: HashSet<Point> = HashSet::new();
    let mut min_row: isize= 0;
    let mut min_col: isize= 0;
    let mut max_row: isize= 0;
    let mut max_col: isize= 0;
    /* 
    for instruction in instructions {
        let dug_out_points = find_points(
            &current_position,
            &instruction.direction,
            &instruction.length
        );
        current_position = *dug_out_points.last().unwrap();
        min_col = min_col.min(current_position.column);
        min_row = min_row.min(current_position.row);
        max_col = max_col.max(current_position.column);
        max_row = max_row.max(current_position.row);
        for point in dug_out_points {
            dug_out.insert(point);
        }
    }
    let dim = Dimensions{
        max_col: max_col,
        min_col: min_col,
        max_row: max_row,
        min_row: min_row
    };*/
    //println!("{:?}", dug_out);
    //draw_map(&dug_out, &dim);
    count_points(&dug_out, &dim) as u32
} 

fn logic_part_2 (input: &Vec<String>) -> u32 {
    1
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_18_unit");
    let result = logic_part_1(&lines);
    assert!(result == 62);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day__unit");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}