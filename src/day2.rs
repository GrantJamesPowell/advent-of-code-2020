use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let file = read_to_string("./src/inputs/day2.txt").expect("can read file");
    let inputs: Vec<(&str, u64)> = file
        .lines()
        .map(|line| line.split(" "))
        .map(|mut words| {
            let direction = words.next().expect("has a direction");
            let val = words.next().expect("has a value");
            (direction, u64::from_str(val).expect("valid number"))
        })
        .collect();

    // Part 1
    let (horizontal, depth): (u64, u64) = inputs.iter().fold(
        (0, 0),
        |(horizontal, depth), &(direction, value)| match direction {
            "forward" => (horizontal + value, depth),
            "up" => (horizontal, depth - value),
            "down" => (horizontal, depth + value),
            other => panic!("invalid direction, {:?}", other),
        },
    );

    println!("day 2 part 1 answer: {:?}", horizontal * depth);

    // Part 2
    let (horizontal, depth, _aim): (u64, u64, u64) = inputs.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), &(direction, value)| match direction {
            "forward" => (horizontal + value, depth + (aim * value), aim),
            "up" => (horizontal, depth, aim - value),
            "down" => (horizontal, depth, aim + value),
            other => panic!("invalid direction, {:?}", other),
        },
    );

    println!("day 2 part 2 answer: {:?}", horizontal * depth);
}
