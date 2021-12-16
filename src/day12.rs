#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::iter::once;

fn main() {
    let file = std::fs::read_to_string("./src/inputs/day12.txt").expect("file_exists");
    let graph = parse(&file);

    let answer = explore(&[], "start", &graph).len();
    println!("Day 12 pt 1: {:?}", answer);
}

fn explore<'a>(
    previous_path: &[&'a str],
    current: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let path_with_current: Vec<_> = previous_path.iter().cloned().chain(once(current)).collect();

    graph[current]
        .iter()
        .flat_map(|next| {
            if is_small_cave(next) && previous_path.contains(next) {
                HashSet::new().into_iter()
            } else if next == &"end" {
                HashSet::<Vec<&str>>::from_iter([vec!["end"]]).into_iter()
            } else {
                explore(&path_with_current, next, graph).into_iter()
            }
        })
        .filter(|x| x.last() == Some(&"end"))
        .map(|mut v| {
            v.insert(0, current);
            v
        })
        .collect()
}

fn is_big_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn parse(file: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    file.lines()
        .map(|line| line.split("-"))
        .for_each(|mut parts| {
            let from = parts.next().expect("has from");
            let to = parts.next().expect("has to");

            let adjacent_from = graph.entry(from).or_insert(vec![]);
            if to != "start" {
                adjacent_from.push(to);
            }

            let adjacent_to = graph.entry(to).or_insert(vec![]);
            if from != "start" {
                adjacent_to.push(from);
            }
        });

    graph
}
