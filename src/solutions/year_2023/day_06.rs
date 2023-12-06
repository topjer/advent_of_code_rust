use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_06");

    println!("Day 6");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn convert_line(line: &String) -> Vec<u32> {
    line.split(":  ")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|x| *x != "")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn convert_line_fixed_kerning(line: &String) -> u64 {
    line.split(":  ")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|x| *x != "")
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn parse_input(input: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let time = convert_line(&input[0]);
    let distance = convert_line(&input[1]);
    println!("{:?}", time);
    println!("{:?}", distance);
    (time, distance)
}

fn parse_input_fixed_kerning(input: &Vec<String>) -> (u64, u64) {
    let time = convert_line_fixed_kerning(&input[0]);
    let distance = convert_line_fixed_kerning(&input[1]);
    println!("{:?}", time);
    println!("{:?}", distance);
    (time, distance)
}
fn logic_part_1 (input: &Vec<String>) -> u32 {
    let (time, distance) = parse_input(input);
    let mut product: u32 = 1;
    for (t, d) in time.iter().zip(distance.iter()) {
        let mut counter: u32 = 0;
        for seconds in 1..=*t {
            if seconds * (t - seconds) > *d {
                counter += 1;
            }
        }
        product *= counter;
    }
    product
}

fn logic_part_2 (input: &Vec<String>) -> u64 {
    let (time, distance) = parse_input_fixed_kerning(input);
    let mut counter: u64 = 0;
    for seconds in 1..=time {
        if seconds * (time - seconds) > distance {
            counter += 1;
        }
    }
    counter
}


#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_06_unit1");
    let result = logic_part_1(&lines);
    assert!(result == 288);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_06_unit1");
    let result = logic_part_2(&lines);
    assert!(result == 71503);
}