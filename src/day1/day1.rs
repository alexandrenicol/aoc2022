use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    const INPUT_FILE : &str = "/workspaces/aoc2022/src/day1/input.csv";

    let file = File::open(INPUT_FILE).unwrap();
    let reader = BufReader::new(file);

    // create an empty array representing the elves
    let mut elves: Vec<Vec<i32>> = Vec::new();

    // init an array for the first elf
    let mut backpack: Vec<i32> = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);

        // check if the line is an empty string
        if line.is_empty() {
            println!("Found an empty string!");
            elves.push(backpack);
            backpack = Vec::new();
        } else {
            backpack.push(line.parse::<i32>().unwrap());
        }


    }

    // calculate the sum of the elves
    let mut sums: Vec<i32> = Vec::new();
    for elf in elves {

        sums.push(elf.iter().sum::<i32>());

        println!("Sum: {}", elf.iter().sum::<i32>());
    }

    // find the max sum
    let max_sum = sums.iter().max().unwrap();
    println!("Max Sum: {}", max_sum);

    // part2

    // order sums from high to low
    sums.sort_by(|a, b| b.cmp(a));

    let top_three = &sums[0..3];
    println!("Top three: {:?}", top_three);

    // sum top_three
    let top_three_sum = top_three.iter().sum::<i32>();
    println!("Top three sum: {}", top_three_sum);


}
