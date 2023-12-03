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

fn process(rel_file_path: &str) -> u64 {
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
        let stars = get_star(value);
        let line_has_star = stars.len() > 0;

        if !line_has_star {
            continue;
        }

        for star in stars {
            let index = star.index;

            let check_previous = *key > 0;
            if check_previous == true {
                let prev_line = map.get(&(key - 1)).expect("Error getting previous line");
                let prev_line_numbers = get_numbers(prev_line);

                for no in prev_line_numbers {
                    let i = no.indeces;
                    if i.contains(&index) {
                        let first_value = no.value.parse::<u64>().unwrap();

                        let max_len = map.len() - 1;
                        let check_next = *key < max_len;
                        if check_next {
                            let next_line = map.get(&(key + 1)).expect("Error getting next line");
                            let next_line_numbers = get_numbers(next_line);

                            for no in next_line_numbers {
                                let i = no.indeces;
                                println!("i: {:?}", i);
                                println!("index: {:?}", index);
                                if i.contains(&index) {
                                    let second_value = no.value.parse::<u64>().unwrap();
                                    part_numbers.push(first_value * second_value);
                                }
                            }
                        }
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
    indeces: Vec<usize>,
}

fn get_numbers(line: &str) -> Vec<Num> {
    let mut b = String::from(line);
    let raw = line
        .split(|v| !char::is_numeric(v))
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    let mut finalized = vec![];
    for val in raw {
        let pivot = b.find(val).unwrap();
        let start = if pivot <= 0 { 0 } else { pivot - 1 };
        let end = start + val.len() + 1;
        let num = Num {
            value: val,
            indeces: (start..end).collect::<Vec<_>>(),
        };
        finalized.push(num);
        let replacement = (0..val.len()).map(|_| ".").collect::<String>();
        b = b.replacen(val, &replacement, 1);
    }

    finalized
}

#[derive(Debug)]
struct Star {
    index: usize,
}

fn get_star(line: &str) -> Vec<Star> {
    let mut b = String::from(line);
    let raw = line
        .split(|v| !is_star(v))
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    let mut finalized = vec![];
    for val in raw {
        let num = Star {
            index: b.find(val).unwrap(),
        };
        finalized.push(num);
        let replacement = (0..val.len()).map(|_| ".").collect::<String>();
        b = b.replacen(val, &replacement, 1);
    }

    println!("finalized: {:?}", finalized);

    finalized
}

fn is_star(c: char) -> bool {
    c == '*'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process("src/bin/input-test-day-3_2.txt"), 467835);
    }
}
