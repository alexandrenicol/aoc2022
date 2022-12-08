use std::fs::File;
use std::io::{BufRead, BufReader};

// rock beats scissors but loses to paper
// paper beats rock but loses to scissors
// scissors beats paper but loses to rock

fn shorthand2_to_points(choice: &str) -> i32 {
    match choice {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => -1,
    }
}

fn choice_needed_to(match_outcome: &str, with_opponent_choice: &str) -> i32 {
    match (with_opponent_choice, match_outcome) {
        ("A", "X") => 3,
        ("B", "X") => 1,
        ("C", "X") => 2,
        ("A", "Y") => 1,
        ("B", "Y") => 2,
        ("C", "Y") => 3,
        ("A", "Z") => 2,
        ("B", "Z") => 3,
        ("C", "Z") => 1,
        _ => -1,
    }
}

fn main() {
    const INPUT_FILE: &str = "/workspaces/aoc2022/src/day2/input.txt";

    let file = File::open(INPUT_FILE).unwrap();
    let reader = BufReader::new(file);

    let mut rounds: Vec<i32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut words = line.split_whitespace();
        let opponent_choice = words.next().unwrap();
        let outcome_needed = words.next().unwrap();

        let points = choice_needed_to(outcome_needed, opponent_choice);

        let outcome_points = shorthand2_to_points(outcome_needed);

        let round_score = outcome_points + points;

        rounds.push(round_score);
    }

    let total_score = rounds.iter().sum::<i32>();
    println!("Total score: {}", total_score);
}
