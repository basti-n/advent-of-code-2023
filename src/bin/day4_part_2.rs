extern crate aoc_utils;

use std::io::{BufRead, BufReader};

use aoc_utils::files::{read_file, resolve_path};

fn main() {
    let rel_file_path = "src/bin/input-day-4.txt";
    let result = process(&rel_file_path);
    println!("Result: {}", result);
}

fn process(rel_file_path: &str) -> u32 {
    let file = read_file(&resolve_path(rel_file_path));

    let mut sum = 0;
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (_index, line) in reader.lines().into_iter().enumerate() {
                let game_info = match &line {
                    Ok(l) => {
                        let parsed_line = parse_line(&l);
                        let line_sum = calculate_line_sum(parsed_line);
                        sum += line_sum;
                    }
                    Err(e) => {
                        println!("Error reading line: {}", e);
                        continue;
                    }
                };
                println!("Game info: {:?}", game_info);
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }

    sum
}

fn calculate_line_sum(line: Vec<Vec<u64>>) -> u32 {
    let base = &line[0];
    let actual = &line[1];

    let line_number_of_matches = base
        .clone()
        .iter()
        .filter_map(|n| {
            if actual.iter().filter(|a| n == *a).count() > 0 {
                Some(1)
            } else {
                None
            }
        })
        .count() as u32;

    if line_number_of_matches == 0 {
        return 0;
    }

    u32::pow(2, line_number_of_matches - 1)
}

fn parse_line(line: &str) -> Vec<Vec<u64>> {
    let game_info = line.split_inclusive(":").nth(1);

    let parsed_lines = match game_info {
        Some(g) => g
            .split("|")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.trim())
            .map(|s| {
                s.split_whitespace()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<_>(),
        None => {
            vec![]
        }
    };

    println!("Parsed lines: {:?}", parsed_lines);
    parsed_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("src/bin/input-test-day-4.txt"), 13);
    }
}
