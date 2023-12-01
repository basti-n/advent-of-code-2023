use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct Digit<'a> {
    value: i32,
    name: &'a str,
}

fn main() {
    println!("Part 2 - Day 1");

    const DIGITS: [Digit; 9] = [
        Digit {
            value: 1,
            name: "one",
        },
        Digit {
            value: 2,
            name: "two",
        },
        Digit {
            value: 3,
            name: "three",
        },
        Digit {
            value: 4,
            name: "four",
        },
        Digit {
            value: 5,
            name: "five",
        },
        Digit {
            value: 6,
            name: "six",
        },
        Digit {
            value: 7,
            name: "seven",
        },
        Digit {
            value: 8,
            name: "eight",
        },
        Digit {
            value: 9,
            name: "nine",
        },
    ];

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

                let str_digits = DIGITS
                    .iter()
                    .map(|digit| digit.name.to_string())
                    .collect::<Vec<String>>();
                let num_digits = DIGITS
                    .iter()
                    .map(|digit| digit.value.to_string())
                    .collect::<Vec<String>>();

                let mut parsed = HashMap::new();

                line.split_whitespace().for_each(|word| {
                    for str_digit in str_digits.iter() {
                        for (index, _) in word.match_indices(str_digit).collect::<Vec<_>>().iter() {
                            parsed.insert(index.to_owned(), str_digit);
                        }
                    }

                    for num_digit in num_digits.iter() {
                        for (index, _) in word.match_indices(num_digit).collect::<Vec<_>>().iter() {
                            parsed.insert(index.to_owned(), num_digit);
                        }
                    }
                });

                let mut mapped = parsed
                    .iter()
                    .map(|(index, value)| (index, value))
                    .collect::<Vec<_>>();

                mapped.sort_by(|a, b| a.0.cmp(b.0));
                let mapped_and_sorted = mapped
                    .iter()
                    .map(|(_, value)| {
                        let value = value
                            .parse::<i32>()
                            .unwrap_or_else(|_| match value.as_ref() {
                                "one" => 1,
                                "two" => 2,
                                "three" => 3,
                                "four" => 4,
                                "five" => 5,
                                "six" => 6,
                                "seven" => 7,
                                "eight" => 8,
                                "nine" => 9,
                                _ => 0,
                            });
                        value
                    })
                    .filter(|value| *value != 0)
                    .collect::<Vec<_>>();

                let (first, last) = match (mapped_and_sorted.first(), mapped_and_sorted.last()) {
                    (Some(first), Some(last)) => (first.to_owned(), last.to_owned()),
                    _ => (0, 0),
                };

                println!("Mapped: {:?}", mapped);
                println!("{}", line);

                let first_and_last = format!("{}{}", first, last).parse::<i32>().unwrap_or(0);
                println!("{}", first_and_last);
                result += first_and_last;
            }
        }
        Err(_) => panic!("File not found!"),
    }

    println!("Result: {}", result);
}
