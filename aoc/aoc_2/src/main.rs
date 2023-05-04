use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum GameOptions {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for GameOptions {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => GameOptions::Rock,
            'B' | 'Y' => GameOptions::Paper,
            _ => GameOptions::Scissors,
        }
    }
}

enum GameScore {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl GameScore {
    fn get_game_score(opponent: &GameOptions, mine: &GameOptions) -> GameScore {
        match (opponent, mine) {
            (GameOptions::Rock, GameOptions::Scissors) => GameScore::Loss,
            (GameOptions::Rock, GameOptions::Paper) => GameScore::Win,
            (GameOptions::Scissors, GameOptions::Rock) => GameScore::Win,
            (GameOptions::Scissors, GameOptions::Paper) => GameScore::Loss,
            (GameOptions::Paper, GameOptions::Rock) => GameScore::Loss,
            (GameOptions::Paper, GameOptions::Scissors) => GameScore::Win,
            (_, _) => GameScore::Draw,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_score(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut score = 0;

    for line in lines {
        let (opponent_char, _, my_char) = line.unwrap().chars().collect_tuple().unwrap();
        let (oponent_option, my_option) =
            (GameOptions::from(opponent_char), GameOptions::from(my_char));
        let game_score = GameScore::get_game_score(&oponent_option, &my_option);
        score += game_score as i32 + my_option as i32;
    }
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
        let score = calculate_score(lines);
        println!("{}", score);
    }
}
