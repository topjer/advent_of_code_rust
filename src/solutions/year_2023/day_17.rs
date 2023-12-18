use crate::solutions::read_file;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_17");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = Vec::new();
    for row in input {
        let mut holder: Vec<usize> = Vec::new();
        row.chars().for_each(|c| {
            holder.push(c.to_string().parse::<usize>().unwrap())        
        });
        res.push(holder);
    }
    //println!("{:?}", res);
    res
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    row: usize,
    column: usize
}

#[derive(Debug, Clone)]
struct Path {
    total_heat: usize,
    last_step: Direction,
    straights: usize,
    current_position: Point,
    total_path: Vec<Point>
}

fn get_heat(map: &Vec<Vec<usize>>, point: &Point ) -> usize {
    map[point.row][point.column]
}
fn possible_directions2(path: &Path) -> Vec<Direction> {
    let mut possible_directions: Vec<Direction> = Vec::new();
    match path.last_step {
        Direction::Right => {
            if path.straights > 3 {
                possible_directions.push(Direction::Up);
                possible_directions.push(Direction::Down);
            }
            if path.straights < 10 {possible_directions.push(Direction::Right)}
        },
        Direction::Left => {
            if path.straights > 3 {
            possible_directions.push(Direction::Up);
            possible_directions.push(Direction::Down);
            }
            if path.straights < 10 {possible_directions.push(Direction::Left)}
        },
        Direction::Up => {
            if path.straights > 3 {
            possible_directions.push(Direction::Left);
            possible_directions.push(Direction::Right);
            }
            if path.straights < 10 {possible_directions.push(Direction::Up)}
        },
        Direction::Down => {
            if path.straights > 3 {
            possible_directions.push(Direction::Left);
            possible_directions.push(Direction::Right);
            }
            if path.straights < 10 {possible_directions.push(Direction::Down)}
        },
    }
    possible_directions
}

fn possible_directions(path: &Path) -> Vec<Direction> {
    let mut possible_directions: Vec<Direction> = Vec::new();
    match path.last_step {
        Direction::Right => {
            possible_directions.push(Direction::Up);
            possible_directions.push(Direction::Down);
            if path.straights < 3 {possible_directions.push(Direction::Right)}
        },
        Direction::Left => {
            possible_directions.push(Direction::Up);
            possible_directions.push(Direction::Down);
            if path.straights < 3 {possible_directions.push(Direction::Left)}
        },
        Direction::Up => {
            possible_directions.push(Direction::Left);
            possible_directions.push(Direction::Right);
            if path.straights < 3 {possible_directions.push(Direction::Up)}
        },
        Direction::Down => {
            possible_directions.push(Direction::Left);
            possible_directions.push(Direction::Right);
            if path.straights < 3 {possible_directions.push(Direction::Down)}
        },
    }
    possible_directions
}

fn allowed_points(map: &Vec<Vec<usize>>, path: &Path, directions: Vec<Direction>) -> Vec<(Direction,Point)> {
    let mut allowed_points: Vec<(Direction,Point)> = Vec::new();
    for dir in directions {
        match dir {
            Direction::Up => {
                if path.current_position.row > 0 {
                    allowed_points.push((dir, Point{row: path.current_position.row - 1, column: path.current_position.column}))
                }
            },
            Direction::Down => {
                if path.current_position.row < map.len() - 1 {
                    allowed_points.push((dir, Point{row: path.current_position.row + 1, column: path.current_position.column}))
                }
            },
            Direction::Left => {
                if path.current_position.column > 0 {
                    allowed_points.push((dir, Point{row: path.current_position.row, column: path.current_position.column - 1 }))
                }
            },
            Direction::Right => {
                if path.current_position.column < map[0].len() - 1 {
                    allowed_points.push((dir, Point{row: path.current_position.row, column: path.current_position.column + 1 }))
                }
            },
        }
    }
    allowed_points
}

fn print_path(map: &Vec<Vec<usize>>, path: &Path) {
    for (r_id, row) in map.iter().enumerate() {
        for (c_id, c) in row.iter().enumerate() {
            if path.total_path.contains(&Point{row: r_id, column: c_id}) {
                print!("O");
            } else {
                print!("{}",c);
            }
        }
        println!()
    }
}

fn logic_part_1 (input: &Vec<String>) -> usize {
    let input_map = parse_input(input);
    let mut paths_to_check: HashMap<usize, Vec<Path>> = HashMap::new();
    let mut shortest_lenghts: HashMap<(Point, Direction, usize), usize> = HashMap::new();
    let mut target_heat: usize = 1;
    let p1 = Point{row: 0, column: 1};
    let p2 = Point{row: 1, column: 0};
    let h1 = get_heat(&input_map, &p1);
    let h2 = get_heat(&input_map, &p2);
    shortest_lenghts.insert((p1, Direction::Right, 1), h1);
    shortest_lenghts.insert((p2, Direction::Down, 1), h1);
    paths_to_check.insert(h1, vec![Path{
        total_heat: h1,
        last_step: Direction::Right,
        current_position: p1,
        straights: 1,
        total_path: vec![Point{row:0, column: 0}, p1]
    }]);
    paths_to_check.insert(h2, vec![Path{
        total_heat: h2,
        last_step: Direction::Down,
        current_position: p2,
        straights: 1,
        total_path: vec![Point{row:0, column: 0}, p2]
    }]);
    while paths_to_check.len() > 0 {
        
        if !paths_to_check.contains_key(&target_heat) || paths_to_check.get(&target_heat).unwrap().len() == 0 {
            target_heat += 1;
            continue;
        }

        let point_to_check = paths_to_check.get_mut(&target_heat).unwrap().pop().unwrap();
        if point_to_check.current_position == (Point{row: input_map.len()-1, column: input_map[0].len()-1}) {
            println!("{:?}", point_to_check);
            print_path(&input_map, &point_to_check);
            //println!("{:?}", point_to_check.total_heat);
            return point_to_check.total_heat;
        }

        let possible_directions = possible_directions(&point_to_check);
        let allowed_points = allowed_points(&input_map, &point_to_check, possible_directions);
        for p in allowed_points {
            let mut new_straights = 1;
            if p.0 == point_to_check.last_step {
                new_straights = point_to_check.straights + 1;
            };
            let new_heat = point_to_check.total_heat + get_heat(&input_map, &p.1);
            if shortest_lenghts.contains_key(&(p.1, p.0, new_straights)) {
                continue;
            } 
            else {
                shortest_lenghts.insert((p.1, p.0, new_straights), new_heat);
            }
            let mut old_path = point_to_check.total_path.clone();
            old_path.push(p.1);
            let new_path = Path {
                    total_heat: new_heat,
                    last_step: p.0,
                    straights: new_straights,
                    current_position: p.1,
                    total_path: old_path
            };
            paths_to_check.entry(new_heat).and_modify(|el|el.push(new_path.clone()
            )).or_insert(vec![new_path.clone()]);
        }
    }
    0
    //println!("{:?}", shortest_lenghts);
    //1
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let input_map = parse_input(input);
    let mut paths_to_check: HashMap<usize, Vec<Path>> = HashMap::new();
    let mut shortest_lenghts: HashMap<(Point, Direction, usize), usize> = HashMap::new();
    let mut target_heat: usize = 1;
    let p1 = Point{row: 0, column: 1};
    let p2 = Point{row: 1, column: 0};
    let h1 = get_heat(&input_map, &p1);
    let h2 = get_heat(&input_map, &p2);
    shortest_lenghts.insert((p1, Direction::Right, 1), h1);
    shortest_lenghts.insert((p2, Direction::Down, 1), h1);
    paths_to_check.insert(h1, vec![Path{
        total_heat: h1,
        last_step: Direction::Right,
        current_position: p1,
        straights: 1,
        total_path: vec![Point{row:0, column: 0}, p1]
    }]);
    paths_to_check.insert(h2, vec![Path{
        total_heat: h2,
        last_step: Direction::Down,
        current_position: p2,
        straights: 1,
        total_path: vec![Point{row:0, column: 0}, p2]
    }]);
    while paths_to_check.len() > 0 {
        
        if !paths_to_check.contains_key(&target_heat) || paths_to_check.get(&target_heat).unwrap().len() == 0 {
            target_heat += 1;
            continue;
        }

        let point_to_check = paths_to_check.get_mut(&target_heat).unwrap().pop().unwrap();
        if point_to_check.current_position == (Point{row: input_map.len()-1, column: input_map[0].len()-1}) {
            println!("{:?}", point_to_check);
            print_path(&input_map, &point_to_check);
            println!("{:?}", point_to_check.total_heat);
            return point_to_check.total_heat;
        }

        let possible_directions = possible_directions2(&point_to_check);
        let allowed_points = allowed_points(&input_map, &point_to_check, possible_directions);
        for p in allowed_points {
            let mut new_straights = 1;
            if p.0 == point_to_check.last_step {
                new_straights = point_to_check.straights + 1;
            };
            let new_heat = point_to_check.total_heat + get_heat(&input_map, &p.1);
            if shortest_lenghts.contains_key(&(p.1, p.0, new_straights)) {
                continue;
            } 
            else {
                shortest_lenghts.insert((p.1, p.0, new_straights), new_heat);
            }
            let mut old_path = point_to_check.total_path.clone();
            old_path.push(p.1);
            let new_path = Path {
                    total_heat: new_heat,
                    last_step: p.0,
                    straights: new_straights,
                    current_position: p.1,
                    total_path: old_path
            };
            paths_to_check.entry(new_heat).and_modify(|el|el.push(new_path.clone()
            )).or_insert(vec![new_path.clone()]);
        }
    }
    0
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_17_unit");
    let result = logic_part_1(&lines);
    assert!(result == 102);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_17_unit");
    let result = logic_part_2(&lines);
    assert!(result == 94);
}

#[test]
fn enum_test()  {
    let mut v = vec![(1,5),(3,9),(4, 3),(6, 2)];
    println!("{:?}", v.iter().map(|x| x.1).min());
    let index_min = v.iter().position(|x| x.1 == v.iter().map(|x| x.1).min().unwrap()).unwrap();
    println!("{:?}", index_min);
    let foo = v.remove(index_min);
    println!("{:?}", v);
    println!("{:?}", foo);
    }

