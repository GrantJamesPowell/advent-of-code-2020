#![feature(array_windows)]

use std::collections::HashMap;

fn main() {
    let (starting, rules) = load_file("./src/inputs/day14.txt");

    // Part 1
    let after = (0..10).fold(starting.clone(), |next, _i| expand(&next, &rules));
    println!("Day 14 Pt 1 answer: {:?}", answer(&after));

    // Part 2
    let after = (0..40).fold(starting, |next, i| {
        println!("Startin {:?}", i);
        expand(&next, &rules)
    });
    println!("Day 14 Pt 2 answer: {:?}", answer(&after));
}

fn answer(expanded: &[char]) -> usize {
    let mut counts = HashMap::new();
    expanded
        .iter()
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);

    let counts: Vec<usize> = counts.into_iter().map(|(_k, v)| v).collect();
    let most_common = counts.iter().max().expect("has at least one");
    let least_common = counts.iter().min().expect("has at least one");

    most_common - least_common
}

fn expand(template: &[char], rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut expanded = Vec::new();
    template
        .array_windows::<2>()
        .for_each(|&[a, b]| match rules.get(&(a, b)) {
            None => expanded.push(a),
            Some(c) => expanded.extend_from_slice(&[a, *c]),
        });

    expanded.push(*template.last().expect("not empty"));
    expanded
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
