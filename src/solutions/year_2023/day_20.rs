use crate::solutions::read_file;
use std::{collections::{HashMap, VecDeque}, env::current_exe};
use num::integer::lcm;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_20");

    println!("Day 20");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pulse {
    // sender -> receiver
    sender: String,
    receiver: String,
    pulse_type: PulseType
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PulseType {
    High,
    Low
}

#[derive(Debug)]
enum Module {
    // list of receivers
    Broadcaster(Vec<String>),
    // list of receivers and state (on or off)
    FlipFlop(Vec<String>, bool),
    // list of receivers and memory which input sent which signal
    Conjunction(Vec<String>, HashMap<String, PulseType>)
}

impl Module {
    fn process_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let mut output: Vec<Pulse> = Vec::new();
        match self {
            Module::Broadcaster(receivers) => {
                output.extend(receivers.iter().map(|x| Pulse{
                    sender: pulse.receiver.clone(),
                    receiver: x.clone(),
                    pulse_type: pulse.pulse_type.clone()
                }).collect::<Vec<Pulse>>())
            },
            Module::FlipFlop(receivers, ref mut state) => {
                match pulse.pulse_type {
                    PulseType::High => {},
                    PulseType::Low => {
                            //new_state = Some(Module::FlipFlop(receivers.clone(), !*state));
                            let old_state = state.clone();
                            *state = !*state;
                            output.extend(receivers.iter().map(|x| Pulse{
                                sender: pulse.receiver.clone(),
                                receiver: x.clone(),
                                pulse_type: if old_state == true {PulseType::Low} else {PulseType::High}
                            }).collect::<Vec<Pulse>>())
                        }
                    }
                },
            Module::Conjunction(receivers, ref mut map) => {
                map.entry(pulse.sender.clone()).and_modify(|x| *x = pulse.pulse_type.clone());
                let mut output_pulse_type = PulseType::High;
                if map.iter().all(|(_, t)| *t == PulseType::High) {
                    output_pulse_type = PulseType::Low;
                }
                output.extend(receivers.iter().map(|x| Pulse{
                    sender: pulse.receiver.clone(),
                    receiver: x.clone(),
                    pulse_type: output_pulse_type.clone()
                }).collect::<Vec<Pulse>>())
            }
        }
        output
    }
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Module> {
    let mut output: HashMap<String, Module> = HashMap::new();
    let mut shortcut: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let recipients: Vec<String> = parts[1].split(", ").map(|x| x.to_string()).collect();
        if parts[0] == "broadcaster" {
            output.insert("broadcaster".to_string(), Module::Broadcaster(recipients.clone()));
            shortcut.insert("broadcaster".to_string(), recipients);
        } else {
            let name: String = parts[0].chars().skip(1).collect();
            shortcut.insert(name.clone(), recipients.clone());
            if parts[0].starts_with("%") {
                output.insert(name, Module::FlipFlop(recipients, false));
            } else if parts[0].starts_with("&") {
                output.insert(name, Module::Conjunction(recipients, HashMap::new()));
            }
        }
    }
    // fill all conjunctions
    for (name, module) in &mut output {
        match  module {
            Module::Conjunction(_, ref mut map) => {
                for (k, v) in &shortcut {
                    if v.contains(name) {
                        map.insert(k.to_string(), PulseType::Low);
                    }
                }
            },
            _ => {}
        }
    }
    output
}


fn logic_part_1 (input: &Vec<String>) -> usize {
    let mut queue: VecDeque<Pulse> = VecDeque::new();
    let mut network = parse_input(input);
    let mut number_low: usize = 0;
    let mut number_high: usize = 0;
    for i in 0..10 {
        // initial button press
        queue.push_back(Pulse{
            sender: "Button".to_string(),
            receiver: "broadcaster".to_string(),
            pulse_type: PulseType::Low
        });
        while !queue.is_empty() {
            let current_pulse = queue.pop_front().unwrap();
            /*
            if (current_pulse.sender == "broadcaster") & 
                (current_pulse.receiver != "zz") {
                    continue;
                }
 */
            if &current_pulse.sender == "jg" {
                if current_pulse.pulse_type.clone() == PulseType::High {
                    println!("Button press: {}", i);
                    println!("{:?} Signal to output from: {}", current_pulse.pulse_type.clone(), current_pulse.sender.clone());
                }
                //continue;
            }
            //println!("{:?}", current_pulse);
            if current_pulse.pulse_type == PulseType::Low {
                number_low += 1;
            } else {
                number_high += 1;
            }
            if current_pulse.receiver == "output" {
                continue;
            }
            if &current_pulse.receiver == "rx" {
            //if &current_pulse.receiver == "rx" && &current_pulse.pulse_type.clone() == &PulseType::Low {
                if current_pulse.pulse_type.clone() == PulseType::Low {
                    println!("Button press: {}", i);
                    println!("{:?} Signal to output from: {}", current_pulse.pulse_type.clone(), current_pulse.sender.clone());
                }
                continue;
            }
            let current_module = network.get_mut(&current_pulse.receiver).unwrap();
            //println!("{:?}", current_module);
            let new_pulses = current_module.process_pulse(&current_pulse);
            //println!("{:?}", current_module);
            queue.extend(new_pulses);
            //println!("{:?}", queue);
            //println!("{:?}", network);
            {
                let foo = network.get(&"zs".to_string()).unwrap();
                //println!("{:?}", foo);
            }
        }
    }
    //println!("high: {}, low: {}", number_high, number_low);
    number_high * number_low
}

fn logic_part_2 (input: &Vec<String>) -> usize{
    /// single low pulse to rx means that mg remembers high pulses from 
    /// all inputs
    /// jg, rh, jm, hf all have to send a high pulse, so not all of their
    /// inputs must sent a high pulse
    /// zs -> jg
    /// nt -> rh
    /// ff -> jm
    /// th -> hf
    /// 
    /// This means not all inputs of zs must sent a high pulse
    /// determined the cycle of high pulses
    /// 3793, 4019, 4003, 3947
    /// 
    let a: usize = lcm(3793, 4019);
    let b: usize = lcm(a, 4003);
    let c: usize = lcm(b, 3947);
    println!("{}", c);
    1
}

#[test]
fn part_two_full_input() {

    let lines = read_file("./src/inputs/year_2023/day_20");
    let result = logic_part_2(&lines);
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_20_unit");
    let result = logic_part_1(&lines);
    assert!(result == 32000000);
}


#[test]
fn test_example_input2() {
    let lines = read_file("./src/inputs/year_2023/day_20_unit2");
    let result = logic_part_1(&lines);
    assert!(result == 11687500);
}

#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_20_unit");
    let result = logic_part_2(&lines);
    assert!(result == 11687500);
}