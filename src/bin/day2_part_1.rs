use std::{
    io::{BufRead, BufReader, Error},
    u16,
};

use aoc_utils::files::{read_file, resolve_path};

impl Line {
    fn new(id: u8) -> Line {
        Line { id, budget: vec![] }
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
    id: u8,
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
struct Budget {
    green: u8,
    blue: u8,
    red: u8,
}

fn main() {
    let rel_file_path = "src/bin/input-day-2.txt";
    let sum = process(rel_file_path);
    println!("Sum: {:?}", sum);
}

fn process(rel_file_path: &str) -> u16 {
    let file = read_file(&resolve_path(rel_file_path)).expect("File not found!");

    let buf_reader = BufReader::new(file);

    const BUDGET: Budget = Budget {
        green: 13,
        blue: 14,
        red: 12,
    };

    let mut possible_ids: Vec<u8> = vec![];

    for line in buf_reader.lines() {
        match parse_line(line) {
            Ok(line) => {
                println!("Line: {:?} || {:?}", line, BUDGET);
                if is_possible(&line, BUDGET) {
                    possible_ids.push(line.id);
                }
            }
            Err(err) => println!("Error parsing line: {}", err),
        }
    }

    println!("Possible ids: {:?}", possible_ids);
    possible_ids.iter().map(|id| *id as u16).sum()
}

fn is_possible(line: &Line, budget: Budget) -> bool {
    for bdg in line.budget.iter() {
        if bdg.green > budget.green {
            return false;
        }

        if bdg.blue > budget.blue {
            return false;
        }

        if bdg.red > budget.red {
            return false;
        }
    }

    true
}

fn parse_line(line: Result<String, Error>) -> Result<Line, Error> {
    match line {
        Err(err) => Err(err),
        Ok(line) => {
            let mut line = line.split_whitespace();

            let id = line
                .nth(1)
                .map(|id| id.trim_end_matches(':'))
                .inspect(|x| println!("Id: {}", x))
                .expect("Cannot find id")
                .parse::<u8>()
                .expect("Error parsing id");

            let counts = line.collect::<Vec<&str>>().join(" ");
            let mut line = Line::new(id);

            let games = counts.split(';');
            for game in games {
                line.add_budget();
                let bdg = line.get_last_budget().unwrap();
                let game = game.split(',').collect::<Vec<&str>>();
                game.iter().for_each(|combo| {
                    println!("Combo: {}", combo);
                    let combo = combo.split_whitespace().collect::<Vec<&str>>();
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

                            println!("Budget: {:?}", bdg);
                        }
                        Err(err) => {
                            println!("Error parsing count: {}", err);
                        }
                    }
                });
            }

            Ok(line)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Game 3: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green, 3 red";
        let line = parse_line(Ok(String::from(line))).unwrap();
        assert_eq!(line.id, 3);
        let budget = line.budget;
        assert_eq!(budget[0].green, 0);
        assert_eq!(budget[0].red, 4);
        assert_eq!(budget[0].blue, 3);
    }

    #[test]
    fn test_is_possible() {
        let line = Line {
            id: 3,
            budget: vec![Budget {
                green: 4,
                blue: 9,
                red: 5,
            }],
        };

        let budget = Budget {
            green: 13,
            blue: 14,
            red: 12,
        };

        assert_eq!(is_possible(&line, budget), true);

        let line = Line {
            id: 3,
            budget: vec![Budget {
                green: 4,
                blue: 15,
                red: 5,
            }],
        };

        let budget = Budget {
            green: 13,
            blue: 14,
            red: 12,
        };

        assert_eq!(is_possible(&line, budget), false);
    }

    #[test]
    fn test_process() {
        let rel_file_path = "src/bin/input-test.txt";
        let result = process(rel_file_path);

        assert_eq!(result, 8);
    }
}
