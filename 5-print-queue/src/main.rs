use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = read_input()?;

    let first = first_solution(&input);
    let second = second_solution(&input);

    println!("First solution: {first}");
    println!("Second solution: {second}");

    Ok(())
}

#[derive(Debug)]
struct Input {
    pairs: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn read_input() -> Result<Input> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    let mut pairs = Vec::new();
    let mut updates = Vec::new();

    let mut switch = false;

    while reader.read_line(&mut buf).is_ok_and(|l| l != 0) {
        if buf.len() == 1 {
            switch = true;
            buf.clear();
            continue;
        }
        buf.pop();
        match switch {
            false => {
                let mut splitted = buf.split("|");
                pairs.push((
                    splitted.next().unwrap().parse()?,
                    splitted.next().unwrap().parse()?,
                ));
            }
            true => {
                let splitted: result::Result<Vec<_>, _> =
                    buf.split(",").map(|s| s.parse::<u32>()).collect();
                updates.push(splitted?);
            }
        }
        buf.clear();
    }

    Ok(Input { pairs, updates })
}

type Page = u32;

// this will be used to check the priority
fn build_map(Input { pairs, .. }: &Input) -> HashMap<Page, HashSet<u32>> {
    // i'm very proud of coming up with the idea of using a adjacency list! I've been strugling
    // with dfs, when i was learning so this is the thing that came to my mind lol
    let mut map = HashMap::new();

    for (left, right) in pairs {
        map.entry(*left)
            .and_modify(|v: &mut HashSet<u32>| {
                v.insert(*right);
            })
            .or_insert(HashSet::from([*right]));
    }

    map
}

fn first_solution(input: &Input) -> u32 {
    let adj_list = build_map(input);
    let mut sum = 0;

    for update in &input.updates {
        let mut cloned = update.clone();
        cloned.sort_by(|f, s| match adj_list[f].contains(s) {
            true => Ordering::Less,
            false => Ordering::Greater,
        });

        if do_vecs_match(update, &cloned) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn second_solution(input: &Input) -> u32 {
    let adj_list = build_map(input);
    let mut sum = 0;

    for update in &input.updates {
        let mut cloned = update.clone();
        cloned.sort_by(|f, s| match adj_list[f].contains(s) {
            true => Ordering::Less,
            false => Ordering::Greater,
        });

        if !do_vecs_match(update, &cloned) {
            sum += cloned[cloned.len() / 2];
        }
    }

    sum
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
