use array_tool::vec::Intersect;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn character_to_alphabet_index(character: char) -> usize {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet_chars: Vec<char> = alphabet.chars().collect();
    let character_chars: Vec<char> = character.to_string().chars().collect();

    let index = alphabet_chars
        .iter()
        .position(|&r| r == character_chars[0])
        .unwrap();

    return index + 1;
}

fn main() {
    const INPUT_FILE: &str = "/workspaces/aoc2022/src/day3/input.txt";

    let file = File::open(INPUT_FILE).unwrap();
    let reader = BufReader::new(file);

    let mut counter = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let left_part = &line[..line.len() / 2];
        let right_part = &line[line.len() / 2..];

        let left_chars: Vec<char> = left_part.chars().collect();
        let right_chars: Vec<char> = right_part.chars().collect();

        let interest_char = left_chars.intersect(right_chars)[0];

        println!("Interest char: {:?}", interest_char);
        let alphabet_index = character_to_alphabet_index(interest_char);
        println!("Alphabet index: {}", alphabet_index);

        counter += alphabet_index;
    }

    println!("Counter: {}", counter);
}
