use crate::solutions::read_file;
use std::collections::HashMap;
use regex::Regex;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_19");

    println!("Day 19");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug)]
enum Operation {
    Bigger(String, usize),
    Smaller(String, usize),
    Redirect
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    target: String
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl Part {
    fn get_value(&self, name: &String) -> usize {
        match name.as_str() {
            "x" => {self.x},
            "m" => {self.m},
            "a" => {self.a},
            "s" => {self.s},
            _ => {println!("unknown attribute"); 0}
        }
    }
}

fn parse_input(input: &Vec<String>) -> (HashMap<String, Vec<Instruction>>, Vec<Part>) {
    let mut part_1: bool = true;
    let mut part_vec: Vec<Part> = Vec::new();
    let mut instruction_map: HashMap<String, Vec<Instruction>> = HashMap::new();
    for line in input {
        if line == "" {
            part_1 = false;
            continue;
        }
        if part_1 == true {
            let re = Regex::new(r"(\w+)\{(.*)\}").unwrap();
            let caps = re.captures(line).ok_or("no match");
            let name: &str = &caps.as_ref().unwrap()[1];
            let instruction_string: &str = &caps.as_ref().unwrap()[2];
            let mut instruction_vec: Vec<Instruction> = Vec::new();
            for instruction in instruction_string.split(',') {
                if instruction.contains(':') {
                    let re2 = Regex::new(r"([xmas])([<|>])(\d*):(\w+)").unwrap();
                    let cap2 = re2.captures(instruction).ok_or("no match");
                    let instruction_type = &cap2.as_ref().unwrap()[1];
                    let instruction_direction = &cap2.as_ref().unwrap()[2];
                    let instruction_quantity = cap2.as_ref().unwrap()[3].parse::<usize>().unwrap();
                    let instruction_target = &cap2.as_ref().unwrap()[4];
                    let mut operation: Operation;
                    if instruction_direction == "<" {
                        operation = Operation::Smaller(instruction_type.to_string(), instruction_quantity);
                    } else {
                        operation = Operation::Bigger(instruction_type.to_string(), instruction_quantity);
                    }
                    instruction_vec.push(Instruction{
                        operation: operation,
                        target: instruction_target.to_string()
                    });
                } else {
                    instruction_vec.push(Instruction{operation: Operation::Redirect,
                    target: instruction.to_string()})
                }
            }
            instruction_map.insert(name.to_string(), instruction_vec);
         } else {
            let part_re =Regex::new(r"[xmas]=(\d+)").unwrap();
            let mut part_captures = part_re.captures_iter(line);
            let part = Part {
                x: part_captures.next().unwrap()[1].parse::<usize>().unwrap(),
                m: part_captures.next().unwrap()[1].parse::<usize>().unwrap(),
                a: part_captures.next().unwrap()[1].parse::<usize>().unwrap(),
                s: part_captures.next().unwrap()[1].parse::<usize>().unwrap(),
            };
            part_vec.push(part);
        }
    }
    (instruction_map, part_vec)
}

fn logic_part_1 (input: &Vec<String>) -> usize {
    let (instructions, parts) = parse_input(input);
    let mut sum: usize = 0;
    //println!("{:?}", instructions);
    //println!("{:?}", parts);
    for part in parts {
        let mut instructionset = "in".to_string();
        'part: loop {
            if instructionset == "A" {
                sum += part.x + part.m + part.a + part.s;
                println!("Accepted");
                break;
            } else if instructionset =="R" {
                println!("Revoked");
                break;
            }
            let ins_set = instructions.get(&instructionset).unwrap();
            //println!("{:?}", ins_set);
            for ins in ins_set {
                match &ins.operation {
                    Operation::Bigger(attribute, number) => {
                        if part.get_value(&attribute) > *number {
                            instructionset = ins.target.clone();
                            continue 'part;
                        }
                    },
                    Operation::Smaller(attribute, number) => {
                        if part.get_value(&attribute) < *number {
                            instructionset = ins.target.clone();
                            continue 'part;
                        }
                    },
                    Operation::Redirect => {
                        instructionset = ins.target.clone();
                        continue 'part;
                    }
                }
            }
        }
    }
    sum
}

#[derive(Debug, Clone)]
struct Path {
    // the interval are open
    workflows: Vec<String>,
    next_step: String,
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Path {
    fn get_interval(&self, name: &String) -> (usize, usize) {
        match name.as_str() {
            "x" => {self.x},
            "m" => {self.m},
            "a" => {self.a},
            "s" => {self.s},
            _ => {println!("unknown attribute"); (0,0)}
        }
    }

    /// It is assumed that value is truly inside the interval, i.e
    /// lower bound < value < upper bound
    fn split_interval(self, field: &String, value: &usize) -> (Path, Path) {
        let mut res1 = self.clone();
        let mut res2 = self.clone();
        match field.as_str() {
            "x" => {
                res1.x = (res1.x.0, *value);
                res2.x = (*value - 1, res2.x.1);
                (res1, res2)
            },
            "m" => {
                res1.m = (res1.m.0, *value);
                res2.m = (*value - 1, res2.m.1);
                (res1, res2)
            },
            "a" => {
                res1.a = (res1.a.0, *value);
                res2.a = (*value - 1, res2.a.1);
                (res1, res2)
            },
            "s" => {
                res1.s = (res1.s.0, *value);
                res2.s = (*value - 1, res2.s.1);
                (res1, res2)
            },
            _ => {println!("unknown attribute"); (res1,res2)}
        }

    }

    /// It is assumed that value is truly inside the interval, i.e
    /// lower bound < value < upper bound
    fn split_interval_bigger(self, field: &String, value: &usize) -> (Path, Path) {
        let mut res1 = self.clone();
        let mut res2 = self.clone();
        match field.as_str() {
            "x" => {
                res1.x = (res1.x.0, *value + 1);
                res2.x = (*value, res2.x.1);
                (res1, res2)
            },
            "m" => {
                res1.m = (res1.m.0, *value + 1);
                res2.m = (*value, res2.m.1);
                (res1, res2)
            },
            "a" => {
                res1.a = (res1.a.0, *value + 1);
                res2.a = (*value, res2.a.1);
                (res1, res2)
            },
            "s" => {
                res1.s = (res1.s.0, *value + 1);
                res2.s = (*value, res2.s.1);
                (res1, res2)
            },
            _ => {println!("unknown attribute"); (res1,res2)}
        }

    }
}
fn logic_part_2 (input: &Vec<String>) -> usize {
    let (instructions, _) = parse_input(input);
    let start = Path {
        workflows: Vec::new(),
        next_step: "in".to_string(),
        x: (0, 4001),
        m: (0, 4001),
        a: (0, 4001),
        s: (0, 4001),
    };
    let mut paths_to_check: Vec<Path> = vec![start];
    let mut accepted_paths: Vec<Path> = Vec::new();
    while paths_to_check.len() > 0 {
        let mut next_path = paths_to_check.pop().unwrap();
        if next_path.next_step == 'A'.to_string() {
            accepted_paths.push(next_path);
            continue;
        }
        if next_path.next_step == 'R'.to_string() {
            continue;
        }
        let mut new_path = next_path.workflows.clone();
        new_path.push(next_path.next_step.clone());
        let instruction = instructions.get(&next_path.next_step).unwrap();
        for ins in instruction {
            println!("{:?}", ins);
            match &ins.operation {
                Operation::Redirect => {
                    let new_path = Path {
                        next_step: ins.target.clone(),
                        workflows: new_path.clone(),
                        ..next_path
                    };
                    paths_to_check.push(new_path);
                },
                Operation::Bigger(field, value) => {
                    let interval = next_path.get_interval(&field);
                    if *value >= interval.1 {
                        // interval does not get through
                    }
                    if interval.0 >= *value {
                        // full interval is contained
                        let new_path = Path {
                            next_step: ins.target.clone(),
                            workflows: new_path.clone(),
                            ..next_path.clone()
                        };
                        paths_to_check.push(new_path);
                    }
                    if interval.0 < *value && *value < interval.1 {
                        let (mut left, mut right) = next_path.split_interval_bigger(field, value);
                        //left.next_step = ins.target.clone();
                        left.workflows = new_path.clone();
                        right.next_step = ins.target.clone();
                        right.workflows = new_path.clone();
                        paths_to_check.push(right);
                        next_path = left;
                    }
                },
                Operation::Smaller(field, value) => {
                    let interval = next_path.get_interval(&field);
                    if *value <= interval.0 {
                        // interval does not get through
                    }
                    if interval.1 <= *value {
                        // full interval is contained
                        let new_path = Path {
                            next_step: ins.target.clone(),
                            workflows: new_path.clone(),
                            ..next_path.clone()
                        };
                        paths_to_check.push(new_path);
                    }
                    if interval.0 < *value && *value < interval.1 {
                        let (mut left, mut right) = next_path.split_interval(field, value);
                        left.next_step = ins.target.clone();
                        left.workflows = new_path.clone();
                        //right.next_step = ins.target.clone();
                        right.workflows = new_path.clone();
                        paths_to_check.push(left);
                        next_path = right;
                    }
                }
            }
        }
        //println!("{:?}", paths_to_check);
        //break;
    }
    println!("{:?}", accepted_paths);
    let mut total_number = 0;
    for path in accepted_paths {
        total_number += (path.x.1 - path.x.0 - 1) * (path.m.1 - path.m.0 - 1) * (path.a.1 - path.a.0 - 1) * (path.s.1 - path.s.0 - 1);
    }
    println!("{}", total_number);
    total_number
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_19_unit");
    let result = logic_part_1(&lines);
    assert!(result == 19114);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_19_unit");
    let result = logic_part_2(&lines);
    assert!(result == 167409079868000);
}

//167409079868000
//154378681970000
//875005389229030