use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Pair {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePairError;

impl FromStr for Pair {
    type Err = ParsePairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once('-').ok_or(ParsePairError)?;
        let x = x_str.parse::<u32>().map_err(|_| ParsePairError)?;
        let y = y_str.parse::<u32>().map_err(|_| ParsePairError)?;
        Ok(Pair { x: x, y: y })
    }
}

fn calc_overlapping_pairs(lines: io::Lines<io::BufReader<File>>) -> Result<u32, ParsePairError> {
    let mut num_of_overlapping = 0;

    for line in lines {
        let line_str = line.unwrap();
        let (left_str_pair, right_str_pair) = line_str.split_once(',').ok_or(ParsePairError)?;
        let left_pair = Pair::from_str(left_str_pair)?;
        let right_pair = Pair::from_str(right_str_pair)?;

        if left_pair.x >= right_pair.x && left_pair.y <= right_pair.y {
            num_of_overlapping += 1;
        } else if right_pair.x >= left_pair.x && right_pair.y <= left_pair.y {
            num_of_overlapping += 1;
        } else {
        }
    }

    Ok(num_of_overlapping)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file path only.");
        return;
    }
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        let overlapping_pairs = calc_overlapping_pairs(lines);
        println!("{}", overlapping_pairs.unwrap());
    }
}
