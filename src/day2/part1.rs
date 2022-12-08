use std::fs::File;
use std::io::{BufRead, BufReader};

// rock beats scissors but loses to paper
// paper beats rock but loses to scissors
// scissors beats paper but loses to rock

fn shorthand_to_choice(choice: &str) -> &str {
    match choice {
        "A" => "rock",
        "B" => "paper",
        "C" => "scissors",
        _ => "rock",
    }
}

fn shorthand_to_choice2(choice: &str) -> &str {
    match choice {
        "X" => "rock",
        "Y" => "paper",
        "Z" => "scissors",
        _ => "rock",
    }
}

fn choice_to_point(choice: &str) -> i32 {
    match choice {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    }
}

fn compare_choices(yours: &str, opponent: &str) -> i32 {
    let yours = choice_to_point(yours);
    let opponent = choice_to_point(opponent);

    if yours == opponent {
        3
    } else if yours == 1 && opponent == 3 {
        6
    } else if yours == 2 && opponent == 1 {
        6
    } else if yours == 3 && opponent == 2 {
        6
    } else {
        0
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
        let opponent = words.next().unwrap();
        let yours = words.next().unwrap();

        let opponent_string = shorthand_to_choice(opponent);
        let yours_string = shorthand_to_choice2(yours);

        let points = choice_to_point(yours_string);

        let result = compare_choices(yours_string, opponent_string);

        let round_score = result + points;

        rounds.push(round_score);
        // println!("{}", round_score);
    }

    let total_score = rounds.iter().sum::<i32>();
    println!("Total score: {}", total_score);
}
