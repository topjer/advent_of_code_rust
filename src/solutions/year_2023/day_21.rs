use crate::solutions::read_file;
use std::{collections::HashMap, collections::VecDeque, collections::HashSet};

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_21");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines, 64);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Game {
    map: Vec<Vec<usize>>,
    // list of step counts and the point you are on
    points_to_check: VecDeque<(usize, Point)>,
    // hashmap that stores for every point the reachable points
    reachable: HashMap<Point, Vec<Point>>
}

impl Game {
    fn run(&mut self, number_of_steps: usize) -> usize {
        let mut at_certain_steps: HashSet<Point> = HashSet::new();
        while !self.points_to_check.is_empty() {
            let current_point = self.points_to_check.pop_front().unwrap();
            if current_point.0 == number_of_steps {
                if !at_certain_steps.contains(&current_point.1) {
                    at_certain_steps.insert(current_point.1.clone());
                }
            }
            if current_point.0 == number_of_steps + 1 {
                break;
            }
            //println!("{:?}", current_point);
            //println!("{:?}", self);
            let new_points = self.find_valid_points(current_point.1);
            new_points.iter().for_each(|x|
                if !self.points_to_check.contains(&(current_point.0+1, *x)) {
                    self.points_to_check.push_back((current_point.0 +1, *x))
        });

            //println!("{:?}", self.reachable);
        }
        println!("{:?}", at_certain_steps.len());
        at_certain_steps.len()
    }

    fn get_value(&self, point: Point) -> usize {
        self.map[point.row][point.column]
    }

    fn find_valid_points(&mut self, point: Point) -> Vec<Point> {
        if self.reachable.contains_key(&point) {
            return self.reachable.get(&point).unwrap().clone();
        } else {
            let mut output: Vec<Point> = Vec::new();
            // check left of point
            if point.column > 0 && self.get_value(Point{row: point.row, column: point.column-1}) == 1 {
                output.push(Point {row: point.row, column: point.column -1})
            }
            // check right of point
            if point.column < self.map[0].len() - 1 && self.get_value(Point{row: point.row, column: point.column+1}) == 1 {
                output.push(Point {row: point.row, column: point.column + 1})
            }
            // check above point
            if point.row > 0 && self.get_value(Point{row: point.row - 1, column: point.column}) == 1 {
                output.push(Point {row: point.row - 1, column: point.column})
            }
            // check below point
            if point.row < self.map.len() - 1 && self.get_value(Point{row: point.row + 1, column: point.column}) == 1 {
                output.push(Point {row: point.row + 1, column: point.column})
            }
            self.reachable.insert(point, output.clone());
            return output;
        }
    }

}

fn parse_input (input: &Vec<String>) -> Game {
    let mut output: Vec<Vec<usize>> = Vec::new();
    let mut start: Point = Point{row: 0,column: 0};
    for (l_id, line) in input.iter().enumerate() {
        let mut row: Vec<usize> = Vec::new();
        for (c_id, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    row.push(1);
                },
                '#' => {
                    row.push(0);
                },
                'S' => {
                    row.push(1);
                    start = Point {row: l_id, column: c_id};
                },
                _ => {
                    println!("Unknown Symbol")
                }
            }
        }
        output.push(row);
    }
    Game{ map: output, points_to_check: VecDeque::from([(0, start)]), reachable: HashMap::new()}
    
}

fn logic_part_1 (input: &Vec<String>, number_of_steps: usize) -> usize {
    let mut game = parse_input(input);
    game.run(number_of_steps)
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    1
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_21_unit");
    let result = logic_part_1(&lines, 6);
    assert!(result == 16);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day__unit");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}

#[test]
fn test_usize() {
    let usize: String= "Hello World".to_string();
    let foo: Vec<usize> = vec![10];
    println!("{}", usize);
    println!("{:?}", foo);
}