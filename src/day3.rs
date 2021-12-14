#![feature(total_cmp)]

mod common;
use std::cmp::Ordering::*;
use std::fs::read_to_string;

fn main() {
    let inputs = load_inputs("./src/inputs/day3.txt");

    let most_common: Vec<bool> = most_common_bit_at_index(inputs.iter());
    let least_common: Vec<bool> = most_common.iter().map(|x| !x).collect();

    // Part 1
    let gamma = common::bits_to_bytes(&most_common);
    let epsilion = common::bits_to_bytes(&least_common);

    println!("Day 3 Pt 1: {:?}", gamma * epsilion);

    // Part 2

    let oxygen_generator = common::bits_to_bytes(find_best_match(inputs.iter(), true));

    let co2_scrubber = common::bits_to_bytes(find_best_match(inputs.iter(), false));

    println!("Day 3 Pt 2: {:?}", oxygen_generator * co2_scrubber)
}

fn load_inputs(file_name: &str) -> Vec<Vec<bool>> {
    let file = read_to_string(file_name).expect("can read file");

    file.lines()
        .map(|line| {
            line.chars()
                .map(|bit| match bit {
                    '1' => true,
                    '0' => false,
                    other => panic!("unexpected input {:?}", other),
                })
                .collect()
        })
        .collect()
}

fn most_common_bit_at_index<'a>(inputs: impl Iterator<Item = &'a Vec<bool>>) -> Vec<bool> {
    let inputs: Vec<&'a Vec<bool>> = inputs.collect();
    let number_of_inputs = inputs.len();
    let input_width = inputs[0].len();

    (0..input_width)
        .map(|idx| inputs.iter().filter(|bits| bits[idx]).count())
        .map(move |count| {
            let ordering = (count as f64).total_cmp(&((number_of_inputs as f64) / 2.0));

            match ordering {
                Less => false,
                Greater | Equal => true,
            }
        })
        .collect()
}

fn find_best_match<'a>(
    inputs: impl Iterator<Item = &'a Vec<bool>>,
    use_most_common: bool,
) -> &'a Vec<bool> {
    let mut inputs: Vec<&Vec<bool>> = inputs.collect();

    let mut current_idx = 0;

    while inputs.len() > 1 {
        let mut most_common = most_common_bit_at_index(inputs.iter().map(|&x| x));

        if !use_most_common {
            for bit in most_common.iter_mut() {
                *bit = !*bit;
            }
        }

        inputs.retain(|bits| bits[current_idx] == most_common[current_idx]);
        current_idx += 1;
    }

    inputs[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bools_to_ints(input: &[bool]) -> Vec<u8> {
        input.iter().map(|&x| if x { 1 } else { 0 }).collect()
    }

    #[test]
    fn example_from_advent_of_code() {
        let inputs = load_inputs("./src/inputs/day3-example.txt");

        // Part 1
        let most_common: Vec<bool> = most_common_bit_at_index(inputs.iter());
        assert_eq!(bools_to_ints(&most_common), vec![1, 0, 1, 1, 0]);
        assert_eq!(common::bits_to_bytes(&most_common), 22);

        let least_common: Vec<bool> = most_common.iter().map(|&x| !x).collect();
        assert_eq!(bools_to_ints(&least_common), vec![0, 1, 0, 0, 1]);
        assert_eq!(common::bits_to_bytes(&least_common), 9);

        // Part 2
        let oxygen = find_best_match(inputs.iter(), true);
        assert_eq!(bools_to_ints(oxygen), vec![1, 0, 1, 1, 1]);

        let carbon = find_best_match(inputs.iter(), false);
        assert_eq!(bools_to_ints(carbon), vec![0, 1, 0, 1, 0]);
    }
}
