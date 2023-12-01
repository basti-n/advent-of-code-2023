use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Part 1 - Day 1");

    let file_path = "src/bin/input-day-1.txt";
    let file = File::open(file_path);

    let mut result: i32 = 0;

    match file {
        Ok(file) => {
            let buf_reader = BufReader::new(file);

            for line in buf_reader.lines() {
                let line = line.unwrap_or_else(|err| {
                    println!("Error reading line: {}", err);
                    return String::from("");
                });

                println!("Line: {}", line);
                let parsed = line
                    .chars()
                    .map(|c| c.to_string())
                    .filter(|a| a.parse::<i32>().is_ok())
                    .collect::<Vec<String>>();

                println!("Parsed: {:?}", parsed);
                let sum: i32 = match (parsed.first(), parsed.last()) {
                    (Some(first), Some(last)) => {
                        let sum_as_str = format!("{}{}", first, last);
                        sum_as_str.parse::<i32>().unwrap_or_else(|err| {
                            println!("Error parsing sum: {}", err);
                            return 0;
                        })
                    }
                    _ => 0,
                };

                println!("Sum: {}", sum);
                result += sum;
            }
        }
        Err(_) => panic!("File not found!"),
    }

    println!("Result: {}", result);
}
