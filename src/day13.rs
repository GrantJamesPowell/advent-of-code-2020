#![feature(int_abs_diff)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let (points, folds) = load_file("./src/inputs/day13.txt");

    // Part 1
    let after_fold = fold_paper(&points, folds[0]);
    println!("Day 13 Pt. 1 answer: {:?}", after_fold.len());

    // Part 2
    let after_fold = folds
        .iter()
        .fold(points, |points, fold| fold_paper(&points, *fold));
    println!("Final points: {:?}", after_fold);
    // I threw the output in google sheets to plot it
    // HZLEHJRK
}

fn fold_paper(points: &HashSet<Point>, fold: Fold) -> HashSet<Point> {
    points.iter().map(|&point| fold.fold_point(point)).collect()
}

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn fold_point(self, point @ (x, y): Point) -> Point {
        match self {
            Fold::X(line) => {
                if x > line {
                    return (line - x.abs_diff(line), y);
                }
            }
            Fold::Y(line) => {
                if y > line {
                    return (x, line - y.abs_diff(line));
                }
            }
        }

        point
    }
}

fn load_file(file_name: &str) -> (HashSet<Point>, Vec<Fold>) {
    let file = std::fs::read_to_string(file_name).expect("file exists");

    let mut folds = Vec::new();
    let mut points = HashSet::new();

    for line in file.lines() {
        if let Some(fold) = line.strip_prefix("fold along ") {
            let mut parts = fold.split("=");
            let direction = parts.next().expect("has direction");
            let line = parts
                .next()
                .map(|num| usize::from_str(num).expect("valid num"))
                .expect("has line");

            match direction {
                "x" => folds.push(Fold::X(line)),
                "y" => folds.push(Fold::Y(line)),
                _ => panic!("invalid direction"),
            }
        } else if !line.is_empty() {
            let mut parts = line
                .split(",")
                .map(|num| usize::from_str(num).expect("valid num"));
            let x = parts.next().expect("has x");
            let y = parts.next().expect("has y");
            points.insert((x, y));
        } else {
            continue;
        }
    }

    (points, folds)
}
