extern crate aoc_utils;

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use aoc_utils::files::{read_file, resolve_path};

fn main() {
    let rel_file_path = "src/bin/input-day-3.txt";
    let result = process(&rel_file_path);
    println!("Result: {}", result);
}

fn process(rel_file_path: &str) -> u32 {
    let file = read_file(&resolve_path(rel_file_path));
    let mut map = HashMap::new();

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().into_iter().enumerate() {
                let line = line.expect("Error reading line").trim().to_string();
                map.insert(index, line);
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }

    let mut part_numbers = vec![];
    for (key, value) in map.iter() {
        let numbers = get_numbers(value);
        let line_has_numbers = numbers.len() > 0;

        if !line_has_numbers {
            continue;
        }

        for number in numbers {
            let index = number.index;
            let len = number.value.len();
            let search_index = index + len;
            let range: Vec<usize> =
                ((if index == 0 { 0 } else { index - 1 })..(search_index + 1)).collect::<Vec<_>>();

            let mut has_match = false;

            let curr_line_range =
                ((if index == 0 { 0 } else { index - 1 })..(search_index + 1)).collect::<Vec<_>>();
            for i in &curr_line_range {
                match value.chars().nth(*i) {
                    Some(c) => {
                        if is_symbol(c) {
                            part_numbers.push(number.value.parse::<u32>().unwrap());
                            has_match = true;
                            break;
                        }
                    }
                    None => {}
                }
            }

            let check_previous = !has_match && *key > 0;
            if check_previous == true {
                let prev_line = map.get(&(key - 1)).expect("Error getting previous line");
                for i in &range {
                    match prev_line.chars().nth(*i) {
                        Some(c) => {
                            if is_symbol(c) {
                                part_numbers.push(number.value.parse::<u32>().unwrap());
                                has_match = true;
                                break;
                            }
                        }
                        None => {}
                    }
                }
            }

            let max_len = map.len() - 1;
            let check_next = !has_match && *key < max_len;
            println!(
                "check_next: {} | (max_len: {}) | (has_match: {})",
                check_next, max_len, has_match
            );
            if check_next == true {
                let next_line = map.get(&(key + 1)).expect("Error getting next line");
                println!(
                    "next_line: {} for range {:?} (number={:?})",
                    next_line, range, number
                );
                for i in &range {
                    match next_line.chars().nth(*i) {
                        Some(c) => {
                            if is_symbol(c) {
                                part_numbers.push(number.value.parse::<u32>().unwrap());
                                break;
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }

    println!("part_numbers: {:?}", part_numbers);
    part_numbers.iter().sum()
}

#[derive(Debug)]
struct Num<'a> {
    value: &'a str,
    index: usize,
}

fn get_numbers(line: &str) -> Vec<Num> {
    let mut b = String::from(line);
    let raw = line
        .split(|v| !is_number(v))
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    let mut finalized = vec![];
    for val in raw {
        let num = Num {
            value: val,
            index: b.find(val).unwrap(),
        };
        finalized.push(num);
        let replacement = (0..val.len()).map(|_| ".").collect::<String>();
        b = b.replacen(val, &replacement, 1);
    }

    finalized
}

fn is_dot(c: char) -> bool {
    c == '.'
}

fn is_number(c: char) -> bool {
    c.is_numeric()
}

fn is_symbol(c: char) -> bool {
    !is_dot(c) && !is_number(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        // assert_eq!(process("src/bin/input-test-day-3.txt"), 4361);
        assert_eq!(process("src/bin/input-test-day-3_1.txt"), 4);
    }
}
