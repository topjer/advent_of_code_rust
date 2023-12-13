use crate::solutions::read_file;
use std::collections::HashMap;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_12");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug)]
struct ConditionRecord {
    instructions: Vec<String>,
    numbers: Vec<usize>
}

impl ConditionRecord {
    fn eliminate_known(&mut self) {
        let mut idx_to_remove: Vec<usize> = Vec::new();
        for (idx, instruction) in self.instructions.iter().enumerate() {
            if instruction.chars().all(|x| x == '1') {
                let temp_len = instruction.len();
                let temp_pos = self.numbers.iter().position(|x| *x == temp_len).unwrap();
                self.numbers.remove(temp_pos);
                idx_to_remove.push(idx);
            }
        }
        idx_to_remove.sort();
        for element in idx_to_remove.iter().rev() {
            self.instructions.remove(*element);
        }
/* 
        if self.instructions[0].chars().nth(0).unwrap() == '1' {
            if self.instructions[0].len() == self.numbers[0] {
                self.instructions.remove(0);
            } else {
                self.instructions[0] = self.instructions[0].chars().skip(self.numbers[0] + 1).collect();
            }
            self.numbers.remove(0);
        }

        if self.instructions.last().chars().rev().nth(0).unwrap() == '1' {

        }
        */
    }

    fn possible_combinations(&self) -> usize {
        if self.instructions.iter().map(|x| x.len()).sum::<usize>() + self.instructions.len() - 1 == self.numbers.iter().sum::<usize>() + self.numbers.len() - 1 {
            return 1
        } else {
            return 0
        }
    }
}


fn solve_game(instructions: &Vec<String>, numbers: &Vec<usize>) -> usize {
    // all numbers have been placed
    if numbers.len() == 0 {
        // no more numbers but still fields to fill
        if instructions.iter().any(|x| x.contains('1')) {
            return 0;
        }
        return 1
    }
    // still numbers to place but no room
    if instructions.len() == 0 && numbers.len() > 0 {
        return 0
    }
    // if our numbers do not fit into the possible instructions, just abort
    if instructions.iter().map(|x| x.len()).sum::<usize>() + instructions.len() - 1
         < numbers.iter().sum::<usize>() + numbers.len() - 1 {
            return 0
        }
    // last number fully fits last slot
    if instructions.last().unwrap().len() == *numbers.last().unwrap() {
        let mut temp_i = instructions.clone();
        let mut temp_n = numbers.clone();
        temp_i.truncate(temp_i.len()-1);
        temp_n.truncate(temp_n.len()-1);
        // if there is any one in the last instruction, I have to remove my number
        if instructions.last().unwrap().contains('1') {
            return solve_game(&temp_i, &temp_n)
        } else {
            // if there is no 1 in the last instruction, also try to fit all numbers in the
            // remaining instructions
            return solve_game(&temp_i, numbers) + solve_game(&temp_i, &temp_n)
        }
    }
    
    // last number does not fit last slot
    if instructions.last().unwrap().len() < *numbers.last().unwrap() {
        let mut temp_i = instructions.clone();
        temp_i.truncate(temp_i.len()-1);
        if instructions.last().unwrap().contains('1') {
            return 0
        } else {
            return solve_game(&temp_i, numbers) 
        }
    }

    // last number partially fits last slot
    if instructions.last().unwrap().len() > *numbers.last().unwrap() {
        // two cases, we try to fit it from the back, or we skip the last character
        // and try to fit it into the rest
        let mut last_instr = instructions.last().unwrap().clone();
        if last_instr.chars().nth_back(*numbers.last().unwrap()).unwrap() == '1' {
            // for a number to fit we would expect that the value at number is 0 because
            // we need an empty space in between, so this is the arm where the number does
            // not fit from the back
            let mut temp_i = instructions.clone();
            temp_i.truncate(temp_i.len()-1);
            let last_char = last_instr.pop();
            temp_i.push(last_instr);
            if last_char.unwrap() == '1' {
                return 0;
            } else {
                return solve_game(&temp_i, numbers);
            }

        } else {
            // source of an error is probably that I pop a '1' and still go with 
            // all numbers into the substring
            let mut temp_i = instructions.clone();
            let mut temp_n = numbers.clone();
            temp_i.truncate(temp_i.len()-1);
            temp_n.truncate(temp_n.len()-1);
            let mut temp_i2 = temp_i.clone();
            let last_char = last_instr.pop().unwrap();
            if last_instr.len() > 0 {
                temp_i.push(last_instr.clone());
            }
            let _ = last_instr.truncate(last_instr.len() - *numbers.last().unwrap());
            if last_instr.len() > 0 {
                temp_i2.push(last_instr);
            }
            if last_char == '1' {
                return solve_game(&temp_i2, &temp_n);
            }
            return solve_game(&temp_i, numbers) + solve_game(&temp_i2, &temp_n);
        }
        println!("not sure why I ended up here");
    }

    1 as usize
}

fn parse_input(input: &Vec<String>, repeat: usize) -> Vec<ConditionRecord> {
    let mut condition_records: Vec<ConditionRecord> = Vec::new();

    for line in input {
        let temp: Vec<&str> = line.split(' ').collect();
        let mut instruction_string = temp[0].replace('?',"0")
                                .replace('#', "1");
        instruction_string.push('0');
        let mut temp_string = instruction_string.repeat(repeat);
        let _ = temp_string.pop();
        let instructions: Vec<String> = temp_string.split('.')
                                    .filter(|x| x.len()>0)
                                    .map(|x| x.to_string())
                                    .collect();
        let numbers: Vec<usize> = temp[1].split(',')
                                .map(|x| x.parse::<usize>().unwrap())
                                .collect();
        let final_numbers: Vec<usize> = numbers.iter().cycle().take(repeat * numbers.len()).map(|x| *x).collect();
        //println!("{:?}", instructions);
        //println!("{:?}", numbers);
        condition_records.push(ConditionRecord{instructions, numbers: final_numbers})
    }
    condition_records
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let mut condition_records = parse_input(input, 1);
    let mut sum: usize = 0;
    for mut line in &mut condition_records {
        //line.eliminate_known();
        //println!("{:?}", line);
        let number_solutions = solve_game(&line.instructions, &line.numbers);
        //println!("Possible solutions: {}", number_solutions);
        sum += number_solutions;
    }
    sum as u32
}

fn logic_part_2 (input: &Vec<String>) -> u64 {
    let mut condition_records = parse_input(input, 5);
    let mut sum: usize = 0;
    for mut line in &mut condition_records {
        println!("{:?}", line);
        let number_solutions = solve_game(&line.instructions, &line.numbers);
        println!("Possible solutions: {}", number_solutions);
        sum += number_solutions;
    }
    sum as u64
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_12_unit");
    let result = logic_part_1(&lines);
    assert!(result == 21);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_12_unit");
    let result = logic_part_2(&lines);
    assert!(result == 525152);
}

#[test]
fn string_experiment() {
    let mut hello = "Hello World".repeat(1);
    //let bar = hello.pop();
    println!("{:?}", hello);
}

#[test]
fn vector_experiment() {
    let mut hello: Vec<u32> = vec![1,2,3,4,5];
    //let temp: Vec<u32> = hello.iter().cycle().take(5 * hello.len()).collect();
    let foo: Vec<u32> = hello.into_iter().cycle().take(10).collect();
    println!("{:?}", foo);
}

#[test]
fn test1() {
    let foo = ConditionRecord { 
        instructions: vec!["0001000001000000".to_string()],
        numbers: vec![11, 1] };
    let res = solve_game(&foo.instructions, &foo.numbers);
    println!("{}", res);
}