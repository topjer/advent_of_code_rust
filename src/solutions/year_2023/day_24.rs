use itertools::Itertools;

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_24");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines, (200000000000000.0,400000000000000.0));
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug)]
struct Hailstone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize)
}

fn parse_input(input: &Vec<String>) -> Vec<Hailstone> {
    let mut output: Vec<Hailstone> = Vec::new();
    for line in input {
        let (pos, v) = line.split(" @ ").collect_tuple().unwrap();
        let position: (isize, isize, isize) = pos.split(", ").into_iter()
            .map(|x| x.trim().parse::<isize>().unwrap()).collect_tuple().unwrap();
        let velocity: (isize, isize, isize) = v.split(", ").into_iter()
            .map(|x| {
                x.trim().parse::<isize>().unwrap()}).collect_tuple().unwrap();
        output.push(Hailstone{ position, velocity})
    }
    println!("{:?}", output);
    output
}

fn logic_part_1 (input: &Vec<String>, dimensions: (f64, f64)) -> usize {
    let mut hailstones = parse_input(input);
    let mut number_intersections: usize = 0;
    while !hailstones.is_empty() {
        let s1 = hailstones.pop().unwrap();
        for s2 in &hailstones {
            let det = s1.velocity.1 * s2.velocity.0 - s1.velocity.0 * s2.velocity.1;
            if det != 0 {
                let z1 = s1.position.0 - s2.position.0;
                let z2 = s1.position.1 - s2.position.1;
                let parameter1: f64 = (s2.velocity.1 * z1 - s2.velocity.0 * z2) as f64 / det as f64;
                let parameter2: f64 = (s1.velocity.1 * z1 - s1.velocity.0 * z2) as f64 / det as f64;
                let int_x: f64 = s1.position.0 as f64 + parameter1 * s1.velocity.0 as f64;
                let int_y: f64 = s1.position.1 as f64 + parameter1 * s1.velocity.1 as f64;
                if dimensions.0 <= int_x && int_x <= dimensions.1 && dimensions.0 <= int_y
                    && int_y <= dimensions.1 && parameter1 >=0.0 && parameter2 >= 0.0{
                    //println!("{:?} and {:?} do intersect", s1, s2);
                    number_intersections += 1;
                }
            }
        }
    }
    number_intersections
}


fn logic_part_2 (input: &Vec<String>) -> u32 {
    1
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_24_unit");
    let result = logic_part_1(&lines, (7.0, 27.0));
    assert!(result == 2);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day__unit");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}

#[test]
fn parse_test () {
    let s1 = "-1";
    let s2 = "1";
    println!("{:?}", s1.parse::<isize>());
    println!("{:?}", s2.parse::<isize>());
}