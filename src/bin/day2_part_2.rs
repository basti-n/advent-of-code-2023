use std::{
    env,
    io::{BufRead, BufReader, Error},
};

use aoc_utils::files::{read_file, resolve_path};

impl Line {
    fn new() -> Line {
        Line { budget: vec![] }
    }

    fn add_budget(&mut self) {
        let bdg = Budget::new();
        self.budget.push(bdg);
    }

    fn get_last_budget(&mut self) -> Option<&mut Budget> {
        self.budget.last_mut()
    }
}
#[derive(Debug)]
struct Line {
    budget: Vec<Budget>,
}

impl Budget {
    fn new() -> Budget {
        Budget {
            green: 0,
            blue: 0,
            red: 0,
        }
    }

    fn add_green(&mut self, green: u8) -> () {
        self.green += green;
    }

    fn add_blue(&mut self, blue: u8) -> () {
        self.blue += blue;
    }

    fn add_red(&mut self, red: u8) {
        self.red += red;
    }
}
#[derive(Debug)]
struct Budget<T: Into<u64> = u8> {
    green: T,
    blue: T,
    red: T,
}

fn main() {
    let rel_file_path = "src/bin/input-day-2.txt";
    let sum = process(rel_file_path);
    println!("Sum: {:?}", sum);
}

fn process(rel_file_path: &str) -> u64 {
    let file = read_file(&resolve_path(rel_file_path)).expect("File not found!");

    let buf_reader = BufReader::new(file);

    let mut highest_cubes: Vec<u64> = vec![];

    for line in buf_reader.lines() {
        match parse_line(line) {
            Ok(line) => {
                let count = get_highest_cube(line);
                highest_cubes.push(count);
            }
            Err(err) => println!("Error parsing line: {}", err),
        }
    }

    highest_cubes.iter().sum::<u64>()
}

fn get_highest_cube(line: Line) -> u64 {
    let mut highest_red = 0;
    let mut highest_blue = 0;
    let mut highest_green = 0;

    for budget in line.budget.iter().map(|x| Budget {
        red: u64::from(x.red),
        blue: u64::from(x.blue),
        green: u64::from(x.green),
    }) {
        if budget.red > highest_red {
            highest_red = budget.red;
        }

        if budget.blue > highest_blue {
            highest_blue = budget.blue;
        }

        if budget.green > highest_green {
            highest_green = budget.green;
        }
    }

    let score: u64 = highest_red * highest_blue * highest_green;
    println!("Line: {:?}", line);
    println!("Score: {}", score);
    score
}

fn parse_line(line: Result<String, Error>) -> Result<Line, Error> {
    match line {
        Err(err) => Err(err),
        Ok(line) => {
            let line = line.split(":").collect::<Vec<&str>>()[1]
                .trim()
                .split_whitespace();

            let counts = line.collect::<Vec<&str>>().join(" ");
            let mut line = Line::new();

            let games = counts.split(';');
            for game in games {
                line.add_budget();
                let bdg = line.get_last_budget().unwrap();
                let game = game.split(',').collect::<Vec<&str>>();
                for combo in game.iter() {
                    println!("Combo: {:?}", combo);
                    let combo = combo.split_whitespace().collect::<Vec<&str>>();
                    println!("Combo[1]: {:?}", combo[0]);
                    let (c, color) = (combo[0].parse::<u8>(), combo[1]);
                    match c {
                        Ok(c) => {
                            if color == "green" {
                                bdg.add_green(c);
                            } else if color == "blue" {
                                bdg.add_blue(c);
                            } else if color == "red" {
                                bdg.add_red(c);
                            }
                        }
                        Err(err) => {
                            println!("Error parsing count: {}", err);
                        }
                    }
                }
            }

            Ok(line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let rel_file_path = "src/bin/input-test.txt";
        let result = process(rel_file_path);

        assert_eq!(result, 2286);
    }
}
