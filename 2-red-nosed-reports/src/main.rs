use std::{
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

type Levels = Vec<u32>;
type Reports = Vec<Levels>;

fn main() -> Result<()> {
    let reports = read_input()?;

    //let reports = test_cases();

    let first = first_solution(&reports);
    let second = second_solution(&reports);

    println!("First solution answer: {first}");
    println!("Second solution answer: {second}");

    Ok(())
}

// for debugging, rust debugger is broken for me so i'm just gonna log it
fn pretty_print(reports: &Reports) {
    for row in reports.iter().take(50) {
        println!("Row: {row:?}");
    }
}

fn read_input() -> Result<Reports> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    let mut vec = Vec::with_capacity(1000);

    while reader.read_line(&mut buf).is_ok_and(|l| l != 0) {
        let splitted: Levels = buf.split_whitespace().flat_map(|s| s.parse()).collect();
        vec.push(splitted);
        buf.clear();
    }

    Ok(vec)
}

fn test_cases() -> Reports {
    vec![
        vec![5, 1, 2, 3, 4, 5],
        vec![48, 46, 47, 49, 51, 54, 56],
        vec![29, 28, 27, 25, 26, 25, 22, 20],
        vec![2, 5, 4, 3, 2],
        vec![1, 1, 2, 3, 4],
        vec![7, 10, 8, 10, 11],
        vec![9, 8, 7, 7, 7],
        vec![1, 3, 2, 4, 5],
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ]
}

fn first_solution(rows: &Reports) -> u32 {
    let mut safe_count = 0;
    for row in rows {
        if is_sorted_special(row) {
            safe_count += 1;
        }
    }

    safe_count
}

fn second_solution(rows: &Reports) -> u32 {
    let mut safe_count = 0;
    for row in rows {
        if is_sorted_special_two(row) {
            println!("{row:?}");
            safe_count += 1;
        }
    }

    safe_count
}

fn is_safe(first: i32, second: i32, is_asc: bool) -> bool {
    let abs_diff = second.abs_diff(first);
    let diff = second - first;

    if !(1..=3).contains(&abs_diff) {
        return false;
    }

    if diff < 0 && is_asc {
        return false;
    }

    if diff > 0 && !is_asc {
        return false;
    }

    true
}

// checks whether the vector is sorted and has increments of 1, 2 and 3
fn is_sorted_special(row: &Levels) -> bool {
    let is_asc = row[0] < row[1];

    let mut prev = row[0];
    for level in row.iter().skip(1) {
        if !is_safe(prev as i32, *level as i32, is_asc) {
            return false;
        }

        prev = *level;
    }

    true
}

fn is_sorted_special_two(row: &Levels) -> bool {
    //for (idx, levels) in row.windows(2).skip(1).enumerate() {
    //    if removed_count > 1 {
    //        return false;
    //    }
    //
    //    let safe_row = is_safe(prev as i32, levels[0] as i32, is_asc);
    //
    //    // if unsafe and removed, does it become safe?
    //    if !safe_row && is_safe(prev as i32, levels[1] as i32, is_asc) {
    //        removed_count += 1;
    //    }
    //
    //    //if !safe_row && is_safe(levels[0] as i32, levels[1] as i32, is_asc) {
    //    //    removed_count += 1;
    //    //}
    //
    //    if !is_unsafe && !safe_row {
    //        is_unsafe = true;
    //    }
    //
    //    prev = levels[0];
    //}
    //
    // !is_unsafe || removed_count == 1

    // i'm just giving up dude, just gonna brute force this
    let mut prev = 0;

    // check if the vec is safe without removing anything
    if is_sorted_special(row) {
        return true;
    }

    for (idx, el) in row.iter().enumerate() {
        let is_safe = is_sorted_special(&Vec::from_iter(
            row.iter()
                .enumerate()
                .filter(|(idx2, _)| idx != *idx2)
                .map(|el| *el.1),
        ));

        if is_safe && (prev != *el) {
            return true;
        }
        prev = *el;
    }

    false
}
