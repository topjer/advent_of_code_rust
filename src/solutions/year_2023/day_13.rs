use num::iter;

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_13");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}
fn parse_input(input: &Vec<String>) -> Vec<Vec<&str>> {
    let mut res: Vec<Vec<&str>> = Vec::new();
    let mut holder: Vec<&str> = Vec::new();
    for line in input {
        if *line == "".to_string() {
            res.push(holder);
            holder = Vec::new();
        } else {
            holder.push(line.as_str());
        }
    } 
    res
}

fn find_horizontal_reflections(plan: &Vec<&str>, allowed_differences: usize) -> usize {
    for i in 1..=plan.len()-1 {
        //println!("i is {i}");
        let back_iter = plan[(2*i as isize - plan.len() as isize).max(0) as usize..i].iter().rev();
        let front_iter = plan[i..(2*i).min(plan.len())].iter();
        //if back_iter.zip(front_iter).all(|(x,y)| x == y) {
        if back_iter.zip(front_iter)
            .map(|(x,y)| 
                x.chars().zip(y.chars()).map(|(xc, yc)|
                    if xc != yc {1} else {0}).sum::<usize>()).sum::<usize>() == allowed_differences {
            println!("Reflection at row {i}");
            return i;
        }
    }
    0
}

fn find_vertical_reflections(plan: &Vec<&str>, allowed_differences: usize) -> usize {
    'a: for i in 1..=plan[0].len()-1 {
        let mut overall_sum: usize = 0;
        for line in plan {
            let back_iter = line[(2*i as isize - line.len() as isize).max(0) as usize..i].chars().rev();
            let front_iter = line[i..(2*i).min(line.len())].chars();
            let partial_sum =  back_iter.zip(front_iter).map(|(x,y)| if x != y {1} else {0} ).sum::<usize>();
            overall_sum += partial_sum;
            if overall_sum > allowed_differences {
                continue 'a;
            }
        }
        if overall_sum == allowed_differences {
            println!("reflection at column {i}");
            return i;
        }
    }
    0
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let plans = parse_input(input);
    let mut sum = 0;
    for plan in plans {
        println!("{:?}", plan);
        sum += 100*find_horizontal_reflections(&plan, 0);
        sum += find_vertical_reflections(&plan, 0);
    }
    sum as u32
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    let plans = parse_input(input);
    let mut sum = 0;
    for plan in plans {
        println!("{:?}", plan);
        sum += 100*find_horizontal_reflections(&plan, 1);
        sum += find_vertical_reflections(&plan, 1);
    }
    sum as u32
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_13_unit");
    let result = logic_part_1(&lines);
    assert!(result == 405);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_13_unit");
    let result = logic_part_2(&lines);
    assert!(result == 400);
}

#[test]
fn vector_test() {
    let v1 = vec![1,2,3];
    let v2 = vec![1,4,5];
    let foo: usize =  v1.iter().zip(v2.iter()).map(|(x,y)| if x != y {1} else {0}).sum();
    println!("{foo}");
}