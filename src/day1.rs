#![feature(array_windows)]

use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    // Read the inputs
    let inputs: Vec<u64> = read_to_string("./src/inputs/day1.txt")
        .expect("can read file")
        .lines()
        .map(|line| u64::from_str(line).expect("valid number"))
        .collect();

    // Part 1
    let answer = inputs
        .array_windows::<2>()
        .filter(|[first, second]| second > first)
        .count();

    println!("The answer to day 1 part 1 is: {}", answer);

    // Part 2
    let sums: Vec<u64> = inputs
        .array_windows::<3>()
        .map(|[a, b, c]| a + b + c)
        .collect();

    let answer = sums
        .array_windows::<2>()
        .filter(|[first, second]| second > first)
        .count();

    println!("The answer to day 1 part 2 is: {}", answer);
}
