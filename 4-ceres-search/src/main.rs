use std::{
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;
    //let input = test_case();

    let first = first_solution(&input);
    let second = second_solution(&input);

    println!("First solution: {first}");
    println!("Second solution: {second}");

    Ok(())
}

fn read_input() -> Result<Vec<Vec<char>>> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut vec = Vec::with_capacity(1000);

    let mut buf = String::new();

    while reader.read_line(&mut buf).is_ok_and(|l| l != 0) {
        vec.push(Vec::from_iter(buf.chars()));
        buf.clear();
    }

    Ok(vec)
}

fn test_case() -> Vec<Vec<char>> {
    vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ]
}

fn check_horizontally(input: &[Vec<char>], row: usize, col: usize) -> bool {
    if col + 3 >= input[row].len() {
        return false;
    }
    if input[row][col] == 'X'
        && input[row][col + 1] == 'M'
        && input[row][col + 2] == 'A'
        && input[row][col + 3] == 'S'
    {
        return true;
    }

    if input[row][col] == 'S'
        && input[row][col + 1] == 'A'
        && input[row][col + 2] == 'M'
        && input[row][col + 3] == 'X'
    {
        return true;
    }

    false
}

fn check_vertically(input: &[Vec<char>], row: usize, col: usize) -> bool {
    if row + 3 >= input.len() {
        return false;
    }

    if input[row][col] == 'X'
        && input[row + 1][col] == 'M'
        && input[row + 2][col] == 'A'
        && input[row + 3][col] == 'S'
    {
        return true;
    }

    if input[row][col] == 'S'
        && input[row + 1][col] == 'A'
        && input[row + 2][col] == 'M'
        && input[row + 3][col] == 'X'
    {
        return true;
    }

    false
}

fn check_diagonally(input: &[Vec<char>], row: usize, col: usize) -> bool {
    if row + 3 >= input.len() || col + 3 >= input[row].len() {
        return false;
    }

    if input[row][col] == 'X'
        && input[row + 1][col + 1] == 'M'
        && input[row + 2][col + 2] == 'A'
        && input[row + 3][col + 3] == 'S'
    {
        return true;
    }

    if input[row][col] == 'S'
        && input[row + 1][col + 1] == 'A'
        && input[row + 2][col + 2] == 'M'
        && input[row + 3][col + 3] == 'X'
    {
        return true;
    }

    false
}

fn check_neg_diagonlly(input: &[Vec<char>], row: usize, col: usize) -> bool {
    if row + 3 >= input.len() || col < 3 {
        return false;
    }

    if input[row][col] == 'X'
        && input[row + 1][col - 1] == 'M'
        && input[row + 2][col - 2] == 'A'
        && input[row + 3][col - 3] == 'S'
    {
        return true;
    }

    if input[row][col] == 'S'
        && input[row + 1][col - 1] == 'A'
        && input[row + 2][col - 2] == 'M'
        && input[row + 3][col - 3] == 'X'
    {
        return true;
    }

    false
}

fn first_solution(input: &[Vec<char>]) -> u32 {
    let mut count = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            // check horizontally
            if check_horizontally(input, row, col) {
                count += 1;
            }

            // check vertically
            if check_vertically(input, row, col) {
                count += 1;
            }

            if check_diagonally(input, row, col) {
                count += 1;
            }

            if check_neg_diagonlly(input, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn check_left(input: &[Vec<char>], row: usize, col: usize) -> bool {
    if row < 1 || col < 1 || row + 1 >= input.len() || col + 1 >= input[row].len() {
        return false;
    }

    if (input[row - 1][col - 1] == 'M' && input[row + 1][col + 1] == 'S')
        || (input[row - 1][col - 1] == 'S' && input[row + 1][col + 1] == 'M')
    {
        return true;
    }

    false
}

fn check_right(input: &[Vec<char>], row: usize, col: usize) -> bool {
    if row < 1 || col < 1 || row + 1 >= input.len() || col + 1 >= input[row].len() {
        return false;
    }

    if (input[row - 1][col + 1] == 'M' && input[row + 1][col - 1] == 'S')
        || (input[row - 1][col + 1] == 'S' && input[row + 1][col - 1] == 'M')
    {
        return true;
    }
    false
}

fn second_solution(input: &[Vec<char>]) -> u32 {
    let mut count = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] == 'A' && check_left(input, row, col) && check_right(input, row, col)
            {
                count += 1;
            }
        }
    }

    count
}
