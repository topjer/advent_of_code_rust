use crate::solutions::read_file;
use std::collections::HashSet;
use itertools::Itertools;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_16");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Beam {
    row: usize,
    column: usize,
    direction: Direction
}

#[derive(PartialEq, PartialOrd, Hash, Eq, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Layout {
    map: Vec<String>,
    visited_fields: HashSet<Beam>,
    fields_to_check: Vec<Beam>,
}

impl Layout {
    fn get_symbol(&self, row: &usize, column: &usize) -> char {
        let row = &self.map[*row];
        row.chars().nth(*column).unwrap()
    }

    fn register_beam(&mut self, beam: Beam) {
        if !self.visited_fields.contains(&beam) {
            self.fields_to_check.push(beam.clone());
            self.visited_fields.insert(beam.clone());
        }
    }

    fn move_in_direction(&mut self, beam: &Beam) {
        match beam.direction {
            Direction::Up => {if beam.row > 0 {
                self.register_beam(Beam{row: beam.row-1, column: beam.column, direction: beam.direction.clone()})
            } else {}},
            Direction::Down => {if beam.row < self.map.len() - 1 {
                self.register_beam(Beam {row: beam.row+1, column: beam.column, direction: beam.direction.clone()});
            } else {}}
            Direction::Left => {if beam.column > 0 {
                self.register_beam(Beam{row: beam.row, column: beam.column-1, direction: beam.direction.clone()});
            } else {}},
            Direction::Right => {if beam.column < self.map.len() - 1 {
                self.register_beam(Beam{row: beam.row, column: beam.column+1, direction: beam.direction.clone()});
            } else {}}
        }
    }

    fn propagate_light(&mut self, beam: &Beam) {
        let symbol = self.get_symbol(&beam.row, &beam.column);
        self.visited_fields.insert(beam.clone());
        //let mut new_beams: Vec<Beam> = Vec::new();
        match symbol {
            '.' => {
                self.move_in_direction(beam);
            },
            '|' => {
                if [Direction::Left, Direction::Right].contains(&beam.direction) {
                    self.move_in_direction(&Beam {direction: Direction::Up, ..beam.clone()});
                    self.move_in_direction(&Beam {direction: Direction::Down, ..beam.clone()});
                } else { self.move_in_direction(beam) }
            },
            '-' => {
                if [Direction::Up, Direction::Down].contains(&beam.direction) {
                    self.move_in_direction(&Beam {direction: Direction::Left, ..beam.clone()});
                    self.move_in_direction(&Beam {direction: Direction::Right, ..beam.clone()});
                } else { self.move_in_direction(beam)}
            },
            '/' => {
                if beam.direction == Direction::Up {
                    self.move_in_direction(&Beam {direction: Direction::Right, ..beam.clone()})
                }
                if beam.direction == Direction::Down {
                    self.move_in_direction(&Beam {direction: Direction::Left, ..beam.clone()})
                }
                if beam.direction == Direction::Right {
                    self.move_in_direction(&Beam {direction: Direction::Up, ..beam.clone()})
                }
                if beam.direction == Direction::Left {
                    self.move_in_direction(&Beam {direction: Direction::Down, ..beam.clone()})
                }
            },
            '\\' => {
                if beam.direction == Direction::Up {
                    self.move_in_direction(&Beam {direction: Direction::Left, ..beam.clone()})
                }
                if beam.direction == Direction::Down {
                    self.move_in_direction(&Beam {direction: Direction::Right, ..beam.clone()})
                }
                if beam.direction == Direction::Right {
                    self.move_in_direction(&Beam {direction: Direction::Down, ..beam.clone()})
                }
                if beam.direction == Direction::Left {
                    self.move_in_direction(&Beam {direction: Direction::Up, ..beam.clone()})
                }
            }
            _ => println!("Not implemented yet")
        }
    }

    fn has_fields_to_check(&self) -> bool {
        return self.fields_to_check.len() > 0
    }

    fn run(&mut self) -> usize {
        while self.has_fields_to_check() {
            let next_beam = self.fields_to_check.pop().unwrap();
            self.propagate_light(&next_beam);
        }
        self.visited_fields.iter().map(|b| (b.row, b.column)).unique().collect::<Vec<(usize, usize)>>().len()
    }

    fn draw(&self) {
        let visited_points = self.visited_fields.iter().map(|b| (b.row, b.column)).unique().collect::<Vec<(usize, usize)>>();
        for (r_id, row) in self.map.iter().enumerate() {
            for (c_id, c) in row.chars().enumerate() {
                if visited_points.contains(&(r_id, c_id)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
}

fn logic_part_1 (input: &Vec<String>) -> usize {
    let mut map = Layout{
        map: input.clone(),
        fields_to_check: vec![Beam{column:0,row: 0,direction: Direction::Right}],
        visited_fields: HashSet::from([Beam{column:0,row: 0,direction: Direction::Right}])
        };
    let number_activated = map.run();
    //map.draw();
    number_activated
}

fn logic_part_2 (input: &Vec<String>) -> usize {
    let mut activation_numbers: Vec<usize> = Vec::new();
    //iterate over all columns
    for i in 0..input.len() {
        let initial_beam = Beam{column:0, row:i, direction: Direction::Right};
        let mut map = Layout{
            map: input.clone(),
            fields_to_check: vec![initial_beam.clone()],
            visited_fields: HashSet::from([initial_beam])
            };
        activation_numbers.push(map.run());
        let initial_beam = Beam{row:i, column:input[0].len()-1, direction: Direction::Left};
        let mut map = Layout{
            map: input.clone(),
            fields_to_check: vec![initial_beam.clone()],
            visited_fields: HashSet::from([initial_beam])
            };
        activation_numbers.push(map.run());
    }
    for i in 0..input[0].len() {
        let initial_beam = Beam{column:i, row:0, direction: Direction::Down};
        let mut map = Layout{
            map: input.clone(),
            fields_to_check: vec![initial_beam.clone()],
            visited_fields: HashSet::from([initial_beam])
            };
        activation_numbers.push(map.run());
        let initial_beam = Beam{row:input.len()-1, column:i, direction: Direction::Up};
        let mut map = Layout{
            map: input.clone(),
            fields_to_check: vec![initial_beam.clone()],
            visited_fields: HashSet::from([initial_beam])
            };
        activation_numbers.push(map.run());
    }
    println!("{:?}", activation_numbers);
    *activation_numbers.iter().max().unwrap()
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_16_unit");
    let result = logic_part_1(&lines);
    assert!(result == 46);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_16_unit");
    let result = logic_part_2(&lines);
    assert!(result == 51);
}

#[test]
fn vector_iteration() {
    let mut foo: Vec<usize> = vec![1,2,3];
    while foo.len() > 0 {
        //let mut new_elements: Vec<usize> = Vec::new();
        let el = foo.pop().unwrap();
        if el % 2 == 0 {
            foo.push(el+1)
        }
        println!("{}", el);
    }
    //foo.extend(new_elements.iter())
    }