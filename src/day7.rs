#![feature(int_abs_diff)]

// Nice little 2000x speed up in release mode
//
// ➜  advent_of_code_2021 git:(master) ✗ time ./target/debug/day7
// Day 7 Pt 1 answer: 342641
// Day 7 Pt 2 answer: 93006301
// ./target/debug/day7  23.38s user 0.08s system 99% cpu 23.458 total
//
// ➜  advent_of_code_2021 git:(master) ✗ time ./target/release/day7
// Day 7 Pt 1 answer: 342641
// Day 7 Pt 2 answer: 93006301
// ./target/release/day7  0.01s user 0.00s system 88% cpu 0.015 total

use std::fs::read_to_string;
use std::str::FromStr;

// https://adventofcode.com/2021/day/7
fn main() {
    let inputs = load_file("./src/inputs/day7.txt");

    // Part 1
    let pos = best_position_with_constant_cost(&inputs);
    let gas_used: usize = inputs.iter().map(|num| num.abs_diff(pos)).sum();
    println!("Day 7 Pt 1 answer: {:?}", gas_used);

    // Part 2
    let pos = best_position_with_increasing_costs(&inputs);
    let gas_used: usize = inputs
        .iter()
        .map(|num| cost_with_increasing_costs(num.abs_diff(pos)))
        .sum();
    println!("Day 7 Pt 2 answer: {:?}", gas_used);
}

fn best_position_with_constant_cost(inputs: &[usize]) -> usize {
    best_position(inputs, |x| x)
}

fn best_position_with_increasing_costs(inputs: &[usize]) -> usize {
    best_position(inputs, cost_with_increasing_costs)
}

fn best_position(inputs: &[usize], cost: impl Fn(usize) -> usize) -> usize {
    let max_pos = inputs.iter().max().expect("at least one input");

    (0..=*max_pos)
        .min_by_key(|pos| {
            inputs
                .iter()
                .map(|num| cost(num.abs_diff(*pos)))
                .sum::<usize>()
        })
        .expect("at least one input")
}

fn cost_with_increasing_costs(distance: usize) -> usize {
    (1..=distance).sum()
}

fn load_file(file_name: &str) -> Vec<usize> {
    read_to_string(file_name)
        .expect("can read file")
        .lines()
        .next()
        .expect("only one line")
        .split(",")
        .map(|num| usize::from_str(num).expect("valid num"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_position() {
        let inputs = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(best_position_with_constant_cost(&inputs), 2);
        assert_eq!(best_position_with_increasing_costs(&inputs), 5);
    }
}
