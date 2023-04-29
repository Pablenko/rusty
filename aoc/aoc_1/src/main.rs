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

fn biggest_sum(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut biggest_sum = 0;
    lines
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i32>().unwrap_or(-1))
        .reduce(|acc, x| {
            if x == -1 {
                if acc > biggest_sum {
                    biggest_sum = acc;
                }
                0
            }
            else {
                acc + x
            }
        })
        .unwrap();
    biggest_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file path only.");
        return;
    }
    let file_path = &args[1];
    if let Ok(lines) = read_lines(file_path) {
        let biggest_sum = biggest_sum(lines);
        println!("{}", biggest_sum);
    }
}
