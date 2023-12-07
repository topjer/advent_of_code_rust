use crate::solutions::read_file;
use core::num;
use std::collections::HashMap;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_07");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

fn parse_input(input: &Vec<String>, hand_order: &HashMap<char, u32>) -> Vec<Hand> {
    let mut game: Vec<Hand> = Vec::new();
    for line in input {
        let split_line: Vec<&str> = line.split(' ').collect();
        let hand = parse_hand(split_line[0], &hand_order);
        let rank = determine_hand_rank(&hand);
        let hand_type = determine_hand_type(&hand);
        game.push(Hand { cards: hand, rank: rank, hand_type: hand_type, bid: split_line[1].parse::<u32>().unwrap()});

    }
    //println!("{:#?} ", &game);
    game
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    rank: u32,
    hand_type: u32,
    bid: u32
}

fn determine_hand_type(hand: &Vec<u32>) -> u32 {
    let mut holder: HashMap<u32, u32> = HashMap::new();
    let mut hand_type: u32 = 0;
    let mut number_jokers: u32 = 0;
    for &c in hand {
        if c == 1 {number_jokers += 1} else {
        *holder.entry(c).or_default() += 1;
        }
    }
    let mut frequencies: Vec<u32> = holder.values().cloned().collect::<Vec<u32>>();
    frequencies.sort();
    frequencies.reverse();
    println!("{:?}", &frequencies);
    if number_jokers == 5 {hand_type = 7} else {
        if frequencies[0] + number_jokers == 5 {hand_type = 7}
        if frequencies[0] + number_jokers == 4 {hand_type = 6}
        if frequencies[0] + number_jokers == 3  {
            if frequencies[1] == 2 {hand_type = 5}
            else {hand_type = 4}
        }
        if frequencies[0] + number_jokers == 2  {
            if frequencies[1] == 2 {hand_type = 3}
            else {hand_type = 2}
        }
        if frequencies[0] + number_jokers == 1  {hand_type = 1}
    }
    hand_type
}

#[test]
fn test_hand_type() {
    assert!(determine_hand_type(&vec![1,1,1,1,1]) == 7);
    assert!(determine_hand_type(&vec![1,1,2,1,1]) == 6);
    assert!(determine_hand_type(&vec![1,1,2,2,1]) == 5);
    assert!(determine_hand_type(&vec![1,1,2,3,1]) == 4);
    assert!(determine_hand_type(&vec![1,1,2,2,3]) == 3);
    assert!(determine_hand_type(&vec![1,1,2,3,4]) == 2);
    assert!(determine_hand_type(&vec![1,5,2,3,4]) == 1);
}

fn determine_hand_rank(hand: &Vec<u32>) -> u32 {
    let mut rank: u32 = 0;
    let base: u32 = 17;
    for (index, &card) in hand.iter().rev().enumerate() {
        rank += card * base.pow(index as u32);
    }
    rank
}

fn parse_hand(hand: &str, hand_order: &HashMap<char, u32>) -> Vec<u32> {
    let mut values: Vec<u32> = Vec::new();
    for c in hand.chars() {
        if c.is_numeric() {
            values.push(c.to_string().parse::<u32>().unwrap())
        } else {
            values.push(*hand_order.get(&c).unwrap())
        }
    }
    values
}

fn normal_order() -> HashMap<char, u32> {
    let mut hand_order: HashMap<char, u32> = HashMap::new();
    hand_order.insert('T', 10);
    hand_order.insert('J', 11);
    hand_order.insert('Q', 12);
    hand_order.insert('K', 13);
    hand_order.insert('A', 14);
    hand_order
}

fn order_joker() -> HashMap<char, u32> {
    let mut hand_order: HashMap<char, u32> = HashMap::new();
    hand_order.insert('T', 10);
    hand_order.insert('J', 1);
    hand_order.insert('Q', 12);
    hand_order.insert('K', 13);
    hand_order.insert('A', 14);
    hand_order
}
fn logic_part_1 (input: &Vec<String>) -> u32 {
    let hand_order = normal_order();
    let mut game = parse_input(input, &hand_order);
    let mut result: u32 = 0;
    game.sort_by_key(|h| (h.hand_type, h.rank));
    for (index, h) in game.iter().enumerate() {
        result += (index as u32 + 1) * h.bid;
    }
    println!("{:?}", game);
    result
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    let hand_order = order_joker();
    let mut game = parse_input(input, &hand_order);
    let mut result: u32 = 0;
    game.sort_by_key(|h| (h.hand_type, h.rank));
    for (index, h) in game.iter().enumerate() {
        result += (index as u32 + 1) * h.bid;
    }
    println!("{:?}", game);
    for h in game {

    println!("{:?}", h);
    }
    result
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_07_unit");
    let result = logic_part_1(&lines);
    assert!(result == 6440);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_07_unit");
    let result = logic_part_2(&lines);
    assert!(result == 5905);
}