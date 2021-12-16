#![feature(array_windows)]

use std::collections::HashMap;

fn main() {
    let (starting, rules) = load_file("./src/inputs/day14.txt");

    // Part 1
    let counts = expand_template(&starting, &rules, 10);
    let min = counts.iter().map(|(_k, v)| v).min().expect("not empty");
    let max = counts.iter().map(|(_k, v)| v).max().expect("not empty");
    println!("Day 14 Pt. 1 answer: {:?}", max - min);

    // Part 1
    let counts = expand_template(&starting, &rules, 40);
    let min = counts.iter().map(|(_k, v)| v).min().expect("not empty");
    let max = counts.iter().map(|(_k, v)| v).max().expect("not empty");
    println!("Day 14 Pt. 2 answer: {:?}", max - min);
}

fn expand_template(
    template: &[char],
    rules: &HashMap<(char, char), char>,
    times: usize,
) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    let mut memoizations = HashMap::new();

    for c in template.iter() {
        *counts.entry(*c).or_insert(0) += 1;
    }

    template.array_windows::<2>().for_each(|&[a, b]| {
        let expanded_counts = expand_pair((a, b), rules, times, &mut memoizations);
        add_counts(&mut counts, &expanded_counts)
    });

    counts
}

fn expand_pair(
    (a, b): (char, char),
    rules: &HashMap<(char, char), char>,
    times: usize,
    memoizations: &mut HashMap<((char, char), usize), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    if times == 0 {
        return HashMap::new();
    }

    if let Some(counts) = memoizations.get(&((a, b), times)) {
        return counts.clone();
    }

    let mut counts = HashMap::new();

    if let Some(c) = rules.get(&(a, b)) {
        *counts.entry(*c).or_insert(0) += 1;

        let left_counts = expand_pair((a, *c), rules, times - 1, memoizations);
        add_counts(&mut counts, &left_counts);
        memoizations.insert(((a, *c), times - 1), left_counts);

        let right_counts = expand_pair((*c, b), rules, times - 1, memoizations);
        add_counts(&mut counts, &right_counts);
        memoizations.insert(((*c, b), times - 1), right_counts);
    }

    counts
}

fn add_counts(to: &mut HashMap<char, usize>, other: &HashMap<char, usize>) {
    for (k, v) in other {
        *to.entry(*k).or_insert(0) += v;
    }
}

fn load_file(file_name: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut starting = Vec::new();
    let mut pairs = HashMap::new();

    std::fs::read_to_string(file_name)
        .expect("file exists")
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            if line.contains(" -> ") {
                let mut parts = line.split(" -> ");
                let from = parts.next().expect("has from");
                let subst = parts.next().expect("has subst");
                let subst = subst.chars().next().expect("is a single char");

                let mut from = from.chars();
                let a = from.next().expect("part a");
                let b = from.next().expect("part b");

                pairs.insert((a, b), subst);
            } else {
                starting.extend(line.chars())
            }
        });

    (starting, pairs)
}
