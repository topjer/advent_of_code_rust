use std::thread::current;
use std::collections::HashSet;

use crate::solutions::read_file;

pub fn solve() {
    let lines = read_file("./src/inputs/year_2023/day_10");

    println!("Day 1");
    let result_part_1 = logic_part_1(&lines);
    println!("The result of the first part is: {}", result_part_1);
    let result_part_2 = logic_part_2(&lines);
    println!("Outcome of the second part is: {}", result_part_2) ;
    println!("------------");
}

#[derive(Debug)]
struct Game {
    position: (usize, usize),
    steps: u32,
    map: Vec<Vec<String>>,
    map_dimensions: (usize, usize),
    direction: String,
    loop_points: HashSet<(usize, usize)>
}


impl Game {
    fn make_initial_move(&mut self) {
        self.steps += 1;

        self.loop_points.insert(self.position);

        // check above
        if self.position.0 > 0 {
            let new_point = (self.position.0-1, self.position.1);
            let field = self.get_field(new_point);
            if ["F".to_string(), "|".to_string(),"7".to_string()].contains(&field) {
                self.position = new_point;
                self.direction = "u".to_string();
                return;
            }
        }

        // check below
        if self.position.0 < self.map_dimensions.0 - 1 {
            let new_point = (self.position.0+1, self.position.1);
            let field = self.get_field(new_point);
            if ["L".to_string(), "|".to_string(),"J".to_string()].contains(&field) {
                self.position = new_point;
                self.direction = "d".to_string();
                return;
            }
        }
        // check right
        if self.position.1 < self.map_dimensions.1 - 1 {
            let new_point = (self.position.0, self.position.1+1);
            let field = self.get_field(new_point);
            if ["7".to_string(), "-".to_string(),"J".to_string()].contains(&field) {
                self.position = new_point;
                self.direction = "r".to_string();
                return;
            }
        }
        // check left
        if self.position.1 > 0 {
            let new_point = (self.position.0, self.position.1-1);
            let field = self.get_field(new_point);
            if ["F".to_string(), "-".to_string(),"L".to_string()].contains(&field) {
                self.position = new_point;
                self.direction = "l".to_string();
                return
            }
        }
    }

    fn move_up(&mut self) {
        self.position = (self.position.0 - 1, self.position.1);
        self.direction = "u".to_string();
    }

    fn move_down(&mut self) {
        self.position = (self.position.0 + 1, self.position.1);
        self.direction = "d".to_string();
    }
    fn move_left(&mut self) {
        self.position = (self.position.0, self.position.1 - 1);
        self.direction = "l".to_string();
    }
    fn move_right(&mut self) {
        self.position = (self.position.0, self.position.1 + 1);
        self.direction = "r".to_string();
    }

    fn make_move(&mut self) {
        let current_field = self.get_current_field();
        self.loop_points.insert((self.position.0, self.position.1));
        if current_field == "|" {
            if self.direction == "u" {self.move_up()} else {self.move_down()}
        }
        if current_field == "-" {
            if self.direction == "r" {self.move_right()} else {self.move_left()}
        }
        if current_field == "L" {
            if self.direction == "d" {self.move_right()} else {self.move_up()}
        }
        if current_field == "J" {
            if self.direction == "r" {self.move_up()} else {self.move_left()}
        }
        if current_field == "7" {
            if self.direction == "r" {self.move_down()} else {self.move_left()}
        }
        if current_field == "F" {
            if self.direction == "l" {self.move_down()} else {self.move_right()}
        }
        self.steps += 1;
    }

    fn run(&mut self) {
        while self.get_current_field() != "S".to_string() {
            self.make_move()
        }

    }

    fn get_field(& self, point: (usize, usize)) -> String {
        self.map[point.0][point.1].clone()
    }

    fn get_current_field(& self) -> String {
        self.get_field(self.position)
    }
    
}

fn parse_input(input: &Vec<String>) -> Game {
    let mut map: Vec<Vec<String>> = Vec::new();
    let mut starting_point: (usize, usize) = (0, 0);
    for (line_id, line) in input.iter().enumerate() {
        let parsed_line: Vec<String> = line.chars().map(|x| x.to_string()).collect();
        let start = line.find('S');
        match start {
            Some(pos) => {starting_point = (line_id, pos)},
            None => {}
        }
        map.push(parsed_line);
    }
    let dimensions = (map.len(), map[0].len());
    Game { position: starting_point, map: map , steps: 0, direction: "".to_string(),
          map_dimensions: dimensions, loop_points: HashSet::new()}
}

fn logic_part_1 (input: &Vec<String>) -> u32 {
    let mut game = parse_input(input);
    //println!("{:?}", game);
    game.make_initial_move();
    //println!("{:?}", game);
    game.run();
    println!("{:?}", game);
    game.steps / 2 as u32
    
}

fn logic_part_2 (input: &Vec<String>) -> u32 {
    let mut game = parse_input(input);
    let mut inner_points: Vec<(usize, usize)> = Vec::new();
    //println!("{:?}", game);
    game.make_initial_move();
    //println!("{:?}", game);
    game.run();
    println!("{:?}", game);
    for row_index in 0..game.map_dimensions.0 {
        let mut inside = false;
        for column_index in 0..game.map_dimensions.1 {
            if game.loop_points.contains(&(row_index, column_index)) {
                if ["|".to_string(), "L".to_string(), "J".to_string(), "S".to_string()].contains(&game.get_field((row_index, column_index))) {
                    inside = !inside;
                } 
            } else {
                if inside == true {
                    //println!("Inside point: {:?}", (row_index, column_index));
                    inner_points.push((row_index, column_index))
                }
            }
        }
    }
    //println!("{:?}", inner_points);
    debug_output(game, &inner_points);
    inner_points.len() as u32
}

fn debug_output(game: Game, inner_points: &Vec<(usize, usize)>) {
    let mut annotated_map: Vec<Vec<String>> = Vec::new();
    for row_count in 0..game.map_dimensions.0 {
        let mut new_row: Vec<String> = Vec::new();
        for col_count in 0..game.map_dimensions.1 {
            let mut c ;
            if game.loop_points.contains(&(row_count, col_count)) {
                c = game.get_field((row_count, col_count));
                match c.as_str() {
                    "J" => c="\u{0230F}".to_string(),
                    "F" => c="\u{0230C}".to_string(),
                    "7" => c="\u{0230D}".to_string(),
                    "L" => c="\u{0230E}".to_string(),
                    _ => c=c
                }
            } else {
                if inner_points.contains(&(row_count, col_count)) {
                    c = "O".to_string();
                } else {
                    c = ".".to_string();
                }
            }
            print!("{}", c);
            new_row.push(c);
        }
        annotated_map.push(new_row);
        println!();
    }
}

#[test]
fn test_example_input() {
    let lines = read_file("./src/inputs/year_2023/day_10_unit");
    let result = logic_part_1(&lines);
    assert!(result == 8);
}


#[test]
fn test_example2_input() {
    let lines = read_file("./src/inputs/year_2023/day_10_unit2");
    let result = logic_part_2(&lines);
    assert!(result == 4);
}

#[test]
fn test_example3_input() {
    let lines = read_file("./src/inputs/year_2023/day_10_unit3");
    let result = logic_part_2(&lines);
    println!("symbol: \u{0231C}" );
    assert!(result == 8);
}