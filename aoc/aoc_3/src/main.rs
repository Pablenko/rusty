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

fn process_rucksack_lines(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let score: u32 = lines
        .map(|line| line.unwrap())
        .map(|line| {
            let str_tups = line.split_at(line.len() / 2);
            (str_tups.0.to_owned(), str_tups.1.to_owned())
        })
        .map(|str_tup| {
            for ch in str_tup.0.chars() {
                if str_tup.1.contains(ch) {
                    return ch;
                }
            }
            '0'
        })
        .map(|ch| {
            if ch.is_ascii_lowercase() {
                ch as u32 - 'a' as u32 + 1
            } else {
                ch.to_ascii_lowercase() as u32 - 'a' as u32 + 27
            }
        })
        .sum();

    score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file path only.");
        return;
    }
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let priority_sum = process_rucksack_lines(lines);
        println!("{}", priority_sum);
    }
}
