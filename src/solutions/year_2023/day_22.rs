use std::collections::{HashMap, HashSet};

use num::bigint;

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_22");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
    id: usize
}

impl Brick {
}

fn parse_input(input: &Vec<String>) -> Vec<Brick> {
    let mut output: Vec<Brick> = Vec::new();
    for (l_id, line) in input.iter().enumerate() {
        let parts: Vec<&str> = line.split('~').collect();
        let numbers: Vec<(&str, &str)> = parts[0].split(',').zip(parts[1].split(',')).collect();
        output.push(Brick {
            id: l_id,
            x: (numbers[0].0.parse::<usize>().unwrap(), numbers[0].1.parse::<usize>().unwrap()),
            y: (numbers[1].0.parse::<usize>().unwrap(), numbers[1].1.parse::<usize>().unwrap()),
            z: (numbers[2].0.parse::<usize>().unwrap(), numbers[2].1.parse::<usize>().unwrap()),
        })
    }
    output.sort_by(|b1,b2| b1.z.0.cmp(&b2.z.0));
    //println!("{:?}", output);
    output
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize
}

fn find_cubes(brick: &Brick) -> Vec<Point> {
    let mut output: Vec<Point> = Vec::new();
    if brick.x.0 == brick.x.1 {
        for r in brick.y.0..=brick.y.1 {
            output.push(Point {
                x: brick.x.0,
                y: r
            })
        }
    } else {
        for r in brick.x.0..=brick.x.1 {
            output.push(Point {
                x: r,
                y: brick.y.0,
            })
        }
    }
    output
}

fn logic_part_1 (input: &Vec<String>) -> usize {
    let bricks = parse_input(input);
    let mut resting_positions: HashMap<Point, (usize, usize)> = HashMap::new();
    let mut is_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut disintegratable: HashSet<usize> = HashSet::new();
    for brick in &bricks {
        let cubes = find_cubes(&brick);
        // height and id
        let bricks_below: Vec<&(usize, usize)>= 
        cubes.iter().filter(|x| resting_positions.contains_key(x))
            .map(|x| { resting_positions.get(x).unwrap() }).collect();
        if bricks_below.is_empty() {
            for cube in cubes {
                resting_positions.insert(cube, (1 + (brick.z.1 - brick.z.0), brick.id));
            }
        } else {
            let max_height = bricks_below.iter().map(|x| x.0).max().unwrap();
            is_supported_by.insert(brick.id, 
                bricks_below.iter().filter(|x| x.0 == max_height)
                                    .map(|b| b.1).collect::<HashSet<usize>>());
            for lower_brick in bricks_below.iter().filter(|x| x.0 ==max_height ){
                supports.entry(lower_brick.1).and_modify(|x| {x.insert(brick.id);}).or_insert(HashSet::from([brick.id]));
            }
            for cube in cubes {
                resting_positions.insert(cube, (max_height + 1 + (brick.z.1 - brick.z.0), brick.id));
            }
        }
    }
    for brick in bricks {
        if !supports.contains_key(&brick.id) {
            disintegratable.insert(brick.id);
        } else if supports.get(&brick.id).unwrap().iter().all(|x| {
            is_supported_by.get(x).unwrap().len() > 1
        }) {
            disintegratable.insert(brick.id);
        }
    }
    println!("{:?}", supports);
    println!("{:?}", is_supported_by);
    disintegratable.len()
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let bricks = parse_input(input);
    let mut resting_positions: HashMap<Point, (usize, usize)> = HashMap::new();
    let mut is_supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut sum: usize = 0;
    for brick in &bricks {
        let cubes = find_cubes(&brick);
        // height and id
        let bricks_below: Vec<&(usize, usize)>= 
        cubes.iter().filter(|x| resting_positions.contains_key(x))
            .map(|x| { resting_positions.get(x).unwrap() }).collect();
        if bricks_below.is_empty() {
            for cube in cubes {
                resting_positions.insert(cube, (1 + (brick.z.1 - brick.z.0), brick.id));
            }
        } else {
            let max_height = bricks_below.iter().map(|x| x.0).max().unwrap();
            is_supported_by.insert(brick.id, 
                bricks_below.iter().filter(|x| x.0 == max_height)
                                    .map(|b| b.1).collect::<HashSet<usize>>());
            for lower_brick in bricks_below.iter().filter(|x| x.0 ==max_height ){
                supports.entry(lower_brick.1).and_modify(|x| {x.insert(brick.id);}).or_insert(HashSet::from([brick.id]));
            }
            for cube in cubes {
                resting_positions.insert(cube, (max_height + 1 + (brick.z.1 - brick.z.0), brick.id));
            }
        }
    }
    let mut number_falling_bricks: HashMap<usize, usize> = HashMap::new();
    for brick in bricks {
        sum += determine_falling_bricks(&brick.id, &is_supported_by, &supports, &mut number_falling_bricks);
    }
    println!("{:?}", supports);
    println!("{:?}", is_supported_by);
    sum 
}

// approach is not correct here because I am only checking
// what would happen if a single block would fall. Instead, I have to check
// whether all supports of a given brick have already fallen
fn determine_falling_bricks(
    brick_id: &usize,
    is_supported_by: &HashMap<usize, HashSet<usize>>,
    supports: &HashMap<usize, HashSet<usize>>,
    number_falling_blocks: &mut HashMap<usize, usize>
) -> usize {
    let mut output = 0;
    if number_falling_blocks.contains_key(&brick_id) {
        return *number_falling_blocks.get(&brick_id).unwrap()
    } else {
        if !supports.contains_key(&brick_id) {
            output = 0;
        } else {
            for upper_brick in supports.get(&brick_id).unwrap() {
                if &is_supported_by.get(upper_brick).unwrap().len() == &1 {
                    output = 1 + determine_falling_bricks(upper_brick, is_supported_by, supports, number_falling_blocks);
                } else {
                    output = 0;
                }
            }
        }
        number_falling_blocks.insert(*brick_id, output);
    }
    println!("brick id: {}, falling blocks: {:?}", brick_id, output);
    output
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_22_unit");
    let result = logic_part_1(&lines);
    assert!(result == 5);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_22_unit");
    let result = logic_part_2(&lines);
    assert!(result == 7);
}

#[test]
fn test_hashset() {
    let mut test: HashSet<usize> = HashSet::new();
    test.insert(1);
    test.insert(2);
    test.insert(1);
    println!("{:?}", test);
}