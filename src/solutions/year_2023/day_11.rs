use std::collections::HashSet;

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_11");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}
fn parse_input(input: &Vec<String>) -> (Vec<(usize, usize)>, (usize, usize)) {
    let mut galaxy_coordinates: Vec<(usize, usize)> = Vec::new();
    for (row_index, line) in input.iter().enumerate() {
        for (column_index, element) in line.chars().enumerate() {
            if element == '#' { 
                galaxy_coordinates.push((row_index, column_index));
            }
        }
    }
    //println!("{:?}", galaxy_coordinates);
    (galaxy_coordinates, (input.len(), input[0].len()))
}

fn find_expansion(coordinates: &Vec<(usize, usize)>, dimensions: (usize, usize)) 
-> (HashSet<usize>, HashSet<usize>) {
    // use hashsets instead of vectors and then difference
    let mut missing_rows: HashSet<usize> = (0..dimensions.0).collect() ;
    let mut missing_columns: HashSet<usize> = (0..dimensions.1).collect() ;
    for galaxy in coordinates {
        let _ = missing_columns.take(&galaxy.1);
        let _ = missing_rows.take(&galaxy.0);
    }
    //println!("{:?}", missing_columns);
    //println!("{:?}", missing_rows);
    (missing_rows, missing_columns)
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let (mut galaxy_coordinates, dimensions)= parse_input(input);
    let (missing_rows, missing_columns) = find_expansion(&galaxy_coordinates, dimensions);
    let mut total_distance: usize = 0;
    while &galaxy_coordinates.len() > &0 {
        let reference_point = galaxy_coordinates.pop().unwrap(); 
        for paired_galaxy in &galaxy_coordinates {
            let distance = reference_point.0.abs_diff(paired_galaxy.0) +
                            reference_point.1.abs_diff(paired_galaxy.1);
            let min_row = reference_point.0.min(paired_galaxy.0);
            let max_row = reference_point.0.max(paired_galaxy.0);
            let min_col = reference_point.1.min(paired_galaxy.1);
            let max_col = reference_point.1.max(paired_galaxy.1);
            let row_offset = missing_rows.iter()
            .filter(|x| (min_row < **x) & (**x < max_row)).count();
            let col_offset = missing_columns.iter()
            .filter(|x| (min_col < **x) & (**x < max_col)).count();
            total_distance += distance + (row_offset + col_offset);
        }
        //println!("{:?}", galaxy_coordinates);
    }
    //println!("{}", total_distance);
    total_distance as u32
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let (mut galaxy_coordinates, dimensions)= parse_input(input);
    let (missing_rows, missing_columns) = find_expansion(&galaxy_coordinates, dimensions);
    let mut total_distance: usize = 0;
    while &galaxy_coordinates.len() > &0 {
        let reference_point = galaxy_coordinates.pop().unwrap(); 
        for paired_galaxy in &galaxy_coordinates {
            let distance = reference_point.0.abs_diff(paired_galaxy.0) +
                            reference_point.1.abs_diff(paired_galaxy.1);
            let min_row = reference_point.0.min(paired_galaxy.0);
            let max_row = reference_point.0.max(paired_galaxy.0);
            let min_col = reference_point.1.min(paired_galaxy.1);
            let max_col = reference_point.1.max(paired_galaxy.1);
            let row_offset = missing_rows.iter()
            .filter(|x| (min_row < **x) & (**x < max_row)).count();
            let col_offset = missing_columns.iter()
            .filter(|x| (min_col < **x) & (**x < max_col)).count();
            total_distance += distance + (row_offset + col_offset) * 999999;
        }
        //println!("{:?}", galaxy_coordinates);
    }
    //println!("{}", total_distance);
    total_distance as usize
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_11_unit");
    let result = logic_part_1(&lines);
    assert!(result == 374);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_11_unit");
    let result = logic_part_2(&lines);
    assert!(result == 8410);
}