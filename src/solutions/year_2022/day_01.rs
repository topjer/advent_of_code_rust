use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2022/day_01");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {:?}", result_part_2) ;
    println!("------------");
}

fn logic_part_1(rations: &Vec<String>) -> u32{
    println!("Hello World");
    let mut sum: u32 = 0;
    let mut max: u32 = 0;
    for element in rations {
        if element == "" {
            if sum > max {
                max = sum;
            };
            sum = 0;
            continue;
        }
        let number: u32 = element.parse().unwrap();
        sum += number;
    }
    max
}

fn logic_part_2(rations: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    let mut all_sums: Vec<u32> = Vec::new();
    for element in rations {
        if element == "" {
            all_sums.push(sum);
            sum = 0;
            continue;
        }
        let number: u32 = element.parse().unwrap();
        sum += number;
    }
    all_sums.sort();
    all_sums.reverse();
    let slice = &all_sums[0..3];
    let result: u32 = slice.iter().sum();
    result
}

#[test]
fn test_input() {
    assert!(logic_part_1(&vec!["1000".to_string(), "200".to_string()]) == 1200);
    assert!(logic_part_1(&vec!["1000".to_string(), "200".to_string(), "".to_string(), "2000".to_string()]) == 2000);
}