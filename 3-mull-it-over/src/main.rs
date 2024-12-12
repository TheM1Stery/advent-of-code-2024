use std::{fs, result};

use regex::Regex;

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    let first = first_solution(&input);
    let second = second_solution(&input);

    println!("First solution: {first}");
    println!("Second solution: {second}");

    Ok(())
}

fn read_input() -> Result<String> {
    let string = fs::read_to_string("input.txt")?;
    Ok(string)
}

fn first_solution(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((?<First>[0-9]+),(?<Second>[0-9]+)\)").unwrap();

    let matches: Vec<_> = regex.captures_iter(input).collect();

    let mut sum = 0;

    for match_mul in matches {
        let first = &match_mul["First"]
            .parse::<u32>()
            .expect("Something wrong with the code or the input");
        let second = &match_mul["Second"]
            .parse::<u32>()
            .expect("Something wrong with the code or the input");

        sum += first * second;
    }

    sum
}

fn second_solution(input: &str) -> u32 {
    let regex = Regex::new(r"mul\((?<First>[0-9]+),(?<Second>[0-9]+)\)|don't\(\)|do\(\)").unwrap();

    let matches: Vec<_> = regex.captures_iter(input).collect();

    let mut sum = 0;

    let mut do_mode = true;

    for match_mul in matches {
        if let Some(m) = match_mul.get(0) {
            match m.as_str() {
                s if s.starts_with("mul") => {
                    if do_mode {
                        let first = &match_mul["First"]
                            .parse::<u32>()
                            .expect("Something wrong with the code or the input");
                        let second = &match_mul["Second"]
                            .parse::<u32>()
                            .expect("Something wrong with the code or the input");

                        sum += first * second;
                    }
                }
                "do()" => do_mode = true,
                "don't()" => do_mode = false,
                _ => unreachable!("It shouldn't happen. Something wrong with regex or input"),
            }
        }
    }

    sum
}
