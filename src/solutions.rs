use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod year_2022;
pub mod year_2023;

pub fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("No such file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.expect("Could not parse line")).collect()
}
