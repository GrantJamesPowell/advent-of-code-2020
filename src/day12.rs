#![allow(dead_code)]
#![feature(array_windows)]

use std::collections::{HashMap, HashSet};
use std::iter::once;

fn main() {
    let file = std::fs::read_to_string("./src/inputs/day12.txt").expect("file_exists");
    let graph = parse(&file);

    // Part 1
    let can_visit_small_caves_once =
        |visited: &[&str], next: &str| !(is_small_cave(next) && visited.contains(&next));

    let answer = explore(&[], "start", &graph, &can_visit_small_caves_once).len();

    println!("Day 12 pt 1: {:?}", answer);

    // Part 2
    let can_visit_a_small_cave_twice = |visited: &[&str], next: &str| {
        if !is_small_cave(next) {
            return true;
        }
        if !visited.contains(&next) {
            return true;
        }
        if matches!(next, "start" | "end") {
            return false;
        }

        let mut counts: HashMap<&str, usize> = HashMap::new();

        visited
            .iter()
            .filter(|cave| is_small_cave(cave))
            .for_each(|cave| {
                let count = counts.entry(cave).or_insert(0);
                *count += 1;
            });

        !counts.iter().any(|(_k, v)| *v > 1)
    };

    let answer = explore(&[], "start", &graph, &can_visit_a_small_cave_twice)
        .into_iter()
        // Hack to deal with buggy code 🤷
        .filter(|path| {
            let mut counts: HashMap<&str, usize> = HashMap::new();
            for small_cave in path.iter().filter(|cave| is_small_cave(cave)) {
                *counts.entry(small_cave).or_insert(0) += 1;
            }
            counts.into_iter().filter(|(_k, v)| *v > 1).count() <= 1
        })
        .count();

    println!("Day 12 pt 2: {:?}", answer);
}

fn explore<'a>(
    previous_path: &[&'a str],
    current: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    can_visit: &impl Fn(&[&str], &str) -> bool,
) -> HashSet<Vec<&'a str>> {
    let path_with_current: Vec<_> = previous_path.iter().cloned().chain(once(current)).collect();

    graph[current]
        .iter()
        .flat_map(|next| {
            if !can_visit(previous_path, next) {
                HashSet::new().into_iter()
            } else if next == &"end" {
                HashSet::<Vec<&str>>::from_iter([vec!["end"]]).into_iter()
            } else {
                explore(&path_with_current, next, graph, can_visit).into_iter()
            }
        })
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
            adjacent_from.push(to);

            let adjacent_to = graph.entry(to).or_insert(vec![]);
            adjacent_to.push(from);
        });

    graph
}
