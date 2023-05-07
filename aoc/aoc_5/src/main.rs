use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn insert_crates_to_cranes(line: &str, cranes: &mut Vec<VecDeque<char>>) {
    for (idx, ch) in line.chars().enumerate() {
        if ch.is_alphabetic() {
            let crane_idx = idx / 4;
            cranes[crane_idx].push_back(ch);
        }
    }
}

fn move_crates(line: &str, cranes: &mut Vec<VecDeque<char>>) {
    let command_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut numeric_comands: Vec<u32> = vec![];
    for capt in command_regex.captures_iter(line) {
        numeric_comands.push(capt[1].parse::<u32>().unwrap());
        numeric_comands.push(capt[2].parse::<u32>().unwrap());
        numeric_comands.push(capt[3].parse::<u32>().unwrap());
    }

    for _ in 0..numeric_comands[0] {
        let from_crane = (numeric_comands[1] - 1) as usize;
        let to_crane = (numeric_comands[2] - 1) as usize;

        if let Some(ch) = cranes[from_crane].pop_front() {
            cranes[to_crane].push_front(ch);
        }
    }
}

fn cranes_state_to_string(cranes: &Vec<VecDeque<char>>) -> String {
    let mut result = "".to_owned();

    for crane in cranes {
        if let Some(front_char) = crane.front() {
            result.push(front_char.clone());
        }
    }
    result
}

fn calculate_crates_order(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut number_of_cranes = 0;
    let mut cranes_vec: Vec<VecDeque<char>> = vec![];

    for line in lines {
        let line = line.unwrap();

        if number_of_cranes == 0 {
            number_of_cranes = (line.len() + 1) / 4;
            for _ in 0..number_of_cranes {
                cranes_vec.push(VecDeque::new());
            }
        }

        if line.starts_with('[') {
            insert_crates_to_cranes(&line, &mut cranes_vec);
        } else if line.starts_with('m') {
            move_crates(&line, &mut cranes_vec);
        }
    }

    return cranes_state_to_string(&cranes_vec);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file path only.");
        return;
    }
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let crates_order = calculate_crates_order(lines);
        println!("{}", crates_order);
    }
}
