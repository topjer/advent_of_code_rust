use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_09");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut holder: Vec<Vec<i32>> = Vec::new();
    for line in input {
        holder.push(line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect());
    }
    println!("{:?}", holder);
    holder
}

fn extrapolate_back(line: Vec<i32>) -> i32 {
    let mut diffs = line.clone();
    let mut sum: i32 = *diffs.last().unwrap();
    loop {
        let temp: Vec<i32> = (0..diffs.len()-1).map(|i| diffs[i+1] - diffs[i]).collect();
        if temp.iter().all(|x| *x == 0) {
            break;
        } else {
            sum += temp.last().unwrap();
        }
        diffs = temp;
        println!("{:?}", diffs);
    }
    //println!("{:?}", sum);
    sum
}

fn extrapolate_front(line: Vec<i32>) -> i32 {
    let mut diffs = line.clone();
    let mut sum: i32 = 0;
    let mut holder: Vec<Vec<i32>> = Vec::new();
    holder.push(line.clone());
    loop {
        let temp: Vec<i32> = (0..diffs.len()-1).map(|i| diffs[i+1] - diffs[i]).collect();
        if temp.iter().all(|x| *x == 0) {
            break;
        } else {
            holder.push(temp.clone());
        }
        diffs = temp;
        //println!("{:?}", diffs);
    }
    for el in holder.iter().rev() {
        sum = el.first().unwrap() - sum;
    }
    //println!("{:?}", holder);
    //println!("{:?}", sum);
    sum
}
fn logic_part_1 (input: &Vec<String>) -> i32 {
    let input_lines = parse_input(input);
    let mut sum = 0;
    for line in input_lines {
        sum += extrapolate_back(line);
    }
    sum
}

fn logic_part_2 (input: &Vec<String>) -> i32 {
    let input_lines = parse_input(input);
    let mut sum = 0;
    for line in input_lines {
        let foo = line.clone();
        sum += extrapolate_front(foo);
    }
    sum
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_09_unit");
    let result = logic_part_1(&lines);
    assert!(result == 114);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_09_unit");
    let result = logic_part_2(&lines);
    assert!(result == 2);
}