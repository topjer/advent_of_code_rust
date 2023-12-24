use crate::solutions::read_file;
use std::collections::{VecDeque, HashMap, HashSet};

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_23");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    row: usize,
    column: usize
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

/// start at 'point' and move in direction 'dir'. From the new point, check 
/// where you can go
fn next_points(point: &Point, dir: &Direction, map: &Vec<String>) -> (Point, Vec<Direction>) {
    let mut new_point: Point;
    let mut new_directions: Vec<Direction> = Vec::new();
    match dir {
        Direction::Down => {
            new_point = Point{row: point.row + 1, column: point.column};
            if [".".to_string(), "<".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Left)
            }
            if [".".to_string(), "v".to_string()].contains(&get_symbol(&Point{row: point.row + 2, column: point.column}, map)) {
                new_directions.push(Direction::Down)
            }
            if [".".to_string(), ">".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Right)
            }
        },
        Direction::Up => {
            new_point = Point{row: point.row - 1, column: point.column};
            if [".".to_string(), "<".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Left)
            }
            if [".".to_string(), "^".to_string()].contains(&get_symbol(&Point{row: point.row - 2, column: point.column}, map)) {
                new_directions.push(Direction::Up)
            }
            if [".".to_string(), ">".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Right)
            }
        },
        Direction::Right => {
            new_point = Point{row: point.row, column: point.column + 1};
            if [".".to_string(), "v".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Down)
            }
            if [".".to_string(), ">".to_string()].contains(&get_symbol(&Point{row: point.row, column: point.column + 2}, map)) {
                new_directions.push(Direction::Right)
            }
            if [".".to_string(), "^".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Up)
            }
        },
        Direction::Left => {
            new_point = Point{row: point.row, column: point.column - 1};
            if [".".to_string(), "v".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Down)
            }
            if [".".to_string(), "^".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Up)
            }
            if [".".to_string(), "<".to_string()].contains(&get_symbol(&Point{row: point.row, column: point.column - 2}, map)) {
                new_directions.push(Direction::Left)
            }
        },
    }
    (new_point, new_directions)
}

fn next_points_easy(point: &Point, dir: &Direction, map: &Vec<String>) -> (Point, Vec<Direction>) {
    let mut new_point: Point;
    let mut new_directions: Vec<Direction> = Vec::new();
    match dir {
        Direction::Down => {
            new_point = Point{row: point.row + 1, column: point.column};
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Left)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row + 2, column: point.column}, map)) {
                new_directions.push(Direction::Down)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Right)
            }
        },
        Direction::Up => {
            new_point = Point{row: point.row - 1, column: point.column};
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Left)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row - 2, column: point.column}, map)) {
                new_directions.push(Direction::Up)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Right)
            }
        },
        Direction::Right => {
            new_point = Point{row: point.row, column: point.column + 1};
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Down)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row, column: point.column + 2}, map)) {
                new_directions.push(Direction::Right)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column + 1}, map)) {
                new_directions.push(Direction::Up)
            }
        },
        Direction::Left => {
            new_point = Point{row: point.row, column: point.column - 1};
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row + 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Down)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row - 1, column: point.column - 1}, map)) {
                new_directions.push(Direction::Up)
            }
            if !["#".to_string()].contains(&get_symbol(&Point{row: point.row, column: point.column - 2}, map)) {
                new_directions.push(Direction::Left)
            }
        },
    }
    (new_point, new_directions)
}

fn get_symbol(point: &Point, map: &Vec<String>) -> String {
    if point.row < map.len() && point.row > 0 {
        map[point.row].chars().nth(point.column).unwrap().to_string()
    } else {
        "#".to_string()
    }
}

fn determine_graph(start: &Point, dir: &Direction, map: &Vec<String>, easy: bool) -> HashMap<Point, HashSet<(Point, usize)>> {
    let mut paths_to_check: VecDeque<(Point, Direction)> = VecDeque::from([(*start, *dir)]);
    let mut checked_paths: HashSet<(Point, Direction)> = HashSet::new();
    // map start to end and distance
    let mut possible_paths: HashMap<Point, HashSet<(Point, usize)>> = HashMap::new();
    while !paths_to_check.is_empty() {
        let mut next_part = paths_to_check.pop_front().unwrap();
        checked_paths.insert(next_part);
        let start_point = next_part.0;
        let points_in_path: Vec<Point> = Vec::new();
        let mut length: usize = 0;
        loop {
            let possible_points = if easy == false {
                next_points(&next_part.0, &next_part.1, map)
            } else {
                next_points_easy(&next_part.0, &next_part.1, map)
            };
            length += 1;
            println!("{:?}", possible_points);
            if !(possible_points.1.len() == 1) {
                possible_paths.entry(start_point).and_modify(|x| {x.insert((possible_points.0, length));})
                    .or_insert(HashSet::from([(possible_points.0, length)]));
                for dir in possible_points.1 {
                    if !checked_paths.contains(&(possible_points.0, dir)) {
                        paths_to_check.push_back((possible_points.0, dir));
                    }
                }
                break;
            } else {
                next_part = (possible_points.0, possible_points.1[0]);
            }
        }
    }
    possible_paths
}



fn logic_part_1 (input: &Vec<String>) -> usize {
    let start = Point{ row: 0, column: input.first().unwrap().find('.').unwrap()};
    let end = Point{ row: input.len() - 1, column: input.last().unwrap().find('.').unwrap()};
    let graph = determine_graph(&start, &Direction::Down, input, false);
    println!("{:?}", graph);
    let mut paths_to_check: Vec<(Point, usize)> = Vec::from([(start, 0)]);
    let mut lengths: Vec<usize> = Vec::new();
    let mut checked_inputs: HashSet<(Point, usize)> = HashSet::new();
    while !paths_to_check.is_empty() {
        let (point, distance) = paths_to_check.pop().unwrap();
        if point == end {
            println!("End reached in {distance} step.");
            lengths.push(distance);
            continue;
        }
        println!("{:?}", point);
        graph.get(&point).unwrap().iter().for_each(|x| {
            if !checked_inputs.contains(&(x.0, x.1 + distance)) {
                paths_to_check.push((x.0, x.1 + distance));
                checked_inputs.insert((x.0, x.1 + distance));
            }
        })
    }
    *lengths.iter().max().unwrap()
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let start = Point{ row: 0, column: input.first().unwrap().find('.').unwrap()};
    let end = Point{ row: input.len() - 1, column: input.last().unwrap().find('.').unwrap()};
    let graph = determine_graph(&start, &Direction::Down, input, true);
    println!("{:?}", graph);
    println!("{:?}", graph.len());
    let mut paths_to_check: Vec<(Point, usize, Vec<Point>)> = Vec::from([(start, 0, Vec::new())]);
    let mut lengths: Vec<usize> = Vec::new();
    let mut checked_inputs: HashSet<(Point, usize, Vec<Point>)> = HashSet::new();
    while !paths_to_check.is_empty() {
        let (point, total_length, previous_points) = paths_to_check.pop().unwrap();
        if point == end {
            println!("End reached in {total_length} step.");
            //println!("{:?}", previous_points);
            lengths.push(total_length);
            continue;
        }
        //println!("{:?}", point);
        if !graph.contains_key(&point) {
            continue;
        }
        graph.get(&point).unwrap().iter().for_each(|(target, distance)| {
            if !checked_inputs.contains(&(*target, distance + total_length, previous_points.clone())) {
                if !previous_points.contains(target) {
                    let mut new_path = previous_points.clone();
                    new_path.push(*target);
                    paths_to_check.push((*target, distance + total_length, new_path));
                    checked_inputs.insert((*target, distance + total_length, previous_points.clone()));
                }
            }
        })
    }
    *lengths.iter().max().unwrap()
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_23_unit");
    let result = logic_part_1(&lines);
    assert!(result == 94);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_23_unit");
    let result = logic_part_2(&lines);
    assert!(result == 154);
}