use std::collections::{HashSet, VecDeque};
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

fn calculate_signal(line: &str) -> usize {
    let mut deq = VecDeque::new();
    let mut hash_set = HashSet::new();

    for (idx, ch) in line.chars().enumerate() {
        deq.push_front(ch);

        if deq.len() == 4 {
            for elem in deq.clone().into_iter() {
                hash_set.insert(elem);
            }

            if hash_set.len() == 4 {
                return idx + 1;
            } else {
                deq.pop_back();
                hash_set.clear();
            }
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file path only.");
        return;
    }
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let line = line.unwrap();
            let signal = calculate_signal(&line);
            println!("{}", signal);
        }
    }
}
