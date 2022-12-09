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

    // init array of string
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();

    let mut counter = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        counter += 1;
        group.push(line);

        if (counter == 3) {
            groups.push(group);
            group = Vec::new();
            counter = 0;
        }
    }

    let mut score = 0;

    for group in groups {
        println!("group: {:?}", group);

        let first_group_chars: Vec<char> = group[0].chars().collect();
        let second_group_chars: Vec<char> = group[1].chars().collect();
        let third_group_chars: Vec<char> = group[2].chars().collect();

        let interest_char = first_group_chars
            .intersect(second_group_chars)
            .intersect(third_group_chars);
        println!("interest_char: {:?}", interest_char);

        let alphabet_index = character_to_alphabet_index(interest_char[0]);
        score += alphabet_index;
    }
    println!("score: {}", score);
}
