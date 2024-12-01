use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let (first, second) = read_input()?;
    //let sol = first_solution(first, second);
    let sol = second_solution(&first, &second);

    println!("Solution: {}", sol);

    Ok(())
}

fn read_input() -> Result<(Vec<u32>, Vec<u32>)> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut tuple = (Vec::new(), Vec::new());

    let mut buf = String::new();

    while reader.read_line(&mut buf).is_ok_and(|x| x != 0) {
        let splitted: Vec<_> = buf.split("   ").collect();
        tuple.0.push(splitted[0].parse()?);
        tuple.1.push(splitted[1][..splitted[1].len() - 1].parse()?);
        buf.clear();
    }

    Ok(tuple)
}

fn first_solution(mut first: Vec<u32>, mut second: Vec<u32>) -> u32 {
    first.sort();
    second.sort();

    let mut sum = 0;

    for (first, second) in first.iter().zip(second) {
        sum += first.abs_diff(second);
    }

    sum
}

fn second_solution(first: &Vec<u32>, second: &Vec<u32>) -> u32 {
    let mut count_map = HashMap::new();

    // collect the count of numbers
    for num in second {
        let ref_count = count_map.entry(num).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut sum = 0;
    for num in first {
        sum += num * count_map.get(num).unwrap_or(&0);
    }

    sum
}
