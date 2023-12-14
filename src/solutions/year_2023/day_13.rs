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

fn find_horizontal_reflections(plan: &Vec<&str>) -> usize {
    for i in 1..=plan.len()-1 {
        //println!("i is {i}");
        let back_iter = plan[(2*i as isize - plan.len() as isize).max(0) as usize..i].iter().rev();
        let front_iter = plan[i..(2*i).min(plan.len())].iter();
        if back_iter.zip(front_iter).all(|(x,y)| x == y) {
            println!("Reflection at row {i}");
            return i;
        }
    }
    0
}

fn find_vertical_reflections(plan: &Vec<&str>) -> usize {
    'a: for i in 1..=plan[0].len()-1 {
        for line in plan {
            let back_iter = line[(2*i as isize - line.len() as isize).max(0) as usize..i].chars().rev();
            let front_iter = line[i..(2*i).min(line.len())].chars();
            if back_iter.zip(front_iter).any(|(x,y)| x != y ) {
                continue 'a;
            }
        }
        println!("reflection at column {i}");
        return i;
    }
    0
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let plans = parse_input(input);
    let mut sum = 0;
    for plan in plans {
        println!("{:?}", plan);
        sum += 100*find_horizontal_reflections(&plan);
        sum += find_vertical_reflections(&plan);
    }
    sum as u32
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    1
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_13_unit");
    let result = logic_part_1(&lines);
    assert!(result == 405);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day__unit");
    let result = logic_part_2(&lines);
    assert!(result == 281);
}