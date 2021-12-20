#![allow(dead_code)]
#![feature(map_first_last)]

// Pretty big speedup with `--release`
//
// ➜  advent_of_code_2021 git:(master) ✗ time ./target/release/day15
// Day 15, pt 1 answer: 583
// Day 15, pt 2 answer: 2934
// ./target/release/day15  3.02s user 0.30s system 99% cpu 3.313 total
// ➜  advent_of_code_2021 git:(master) ✗ time ./target/debug/day15
// Day 15, pt 1 answer: 583
// Day 15, pt 2 answer: 2934
// ./target/debug/day15  148.27s user 0.74s system 99% cpu 2:29.12 total

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter::once,
    str::FromStr,
};

fn main() {
    let board = load_file("./src/inputs/day15-example.txt");

    // Part 1
    println!("Day 15, pt 1 answer: {:?}", find_answer(&board, 1));

    // Part 2
    println!("Day 15, pt 2 answer: {:?}", find_answer(&board, 5));
}

fn find_answer(board: &Board, multiplier: usize) -> usize {
    let mut best_path: HashMap<Position, Vec<Position>> = HashMap::from_iter([((0, 0), vec![])]);
    let mut unexplored: BTreeSet<Position> = BTreeSet::from_iter([(0, 0)]);
    let mut explored: HashSet<Position> = HashSet::new();

    while let Some(pos) = unexplored.pop_first() {
        explored.insert(pos);

        for neighbor in board.neighbors(pos, multiplier) {
            let p: Vec<Position> = best_path[&pos].iter().cloned().chain(once(pos)).collect();
            let p_cost = board.path_cost(p.iter().cloned());

            if !explored.contains(&neighbor) {
                unexplored.insert(neighbor);
            }

            match best_path.get(&neighbor) {
                Some(assumed_best) => {
                    let current_score = board.path_cost(assumed_best.iter().cloned());
                    if current_score > p_cost {
                        best_path.insert(neighbor, p);
                    }
                }
                None => {
                    best_path.insert(neighbor, p);
                }
            }
        }
    }

    let optimal = &best_path[&board.goal(multiplier)];

    board.path_cost(optimal[1..].iter().cloned()) + board.risk_at(board.goal(multiplier))
}

type Position = (usize, usize);

struct Board(Vec<Vec<usize>>);

impl Board {
    fn path_cost(&self, path: impl Iterator<Item = Position>) -> usize {
        path.map(|pos| self.risk_at(pos)).sum()
    }

    fn risk_at(&self, (x, y): Position) -> usize {
        let real_width = x % self.real_width();
        let real_height = y % self.real_height();
        let multipler = (x / self.real_width()) + (y / self.real_height());
        let original = self.0[real_height][real_width];

        let mut risk = original + multipler;

        while risk > 9 {
            risk -= 9;
        }

        risk
    }

    fn goal(&self, multiplier: usize) -> Position {
        (self.width(multiplier) - 1, self.height(multiplier) - 1)
    }

    fn neighbors(&self, (x, y): Position, multiplier: usize) -> impl Iterator<Item = Position> {
        let x = x as isize;
        let y = y as isize;

        let width = self.width(multiplier) as isize;
        let height = self.height(multiplier) as isize;

        [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
            .into_iter()
            .filter(move |&(x, y)| 0 <= x && x < width && 0 <= y && y < height)
            .map(|(x, y)| (x as usize, y as usize))
    }

    fn width(&self, multiplier: usize) -> usize {
        self.real_width() * multiplier
    }

    fn height(&self, multiplier: usize) -> usize {
        self.real_height() * multiplier
    }

    fn real_width(&self) -> usize {
        self.0[0].len()
    }

    fn real_height(&self) -> usize {
        self.0.len()
    }
}

fn load_file(file_name: &str) -> Board {
    let board = std::fs::read_to_string(file_name)
        .expect("file exists")
        .lines()
        .map(|line| {
            line.split("")
                .filter(|piece| !piece.is_empty())
                .map(|num| usize::from_str(num).expect("valid num"))
                .collect::<Vec<_>>()
        })
        .collect();

    Board(board)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_at() {
        let board = load_file("./src/inputs/day15-example.txt");
        let risks = std::fs::read_to_string("./src/inputs/day15-example-risk-map.txt")
            .expect("file exists");

        for (y, line) in risks.lines().enumerate() {
            for (x, risk) in line.split("").filter(|x| !x.is_empty()).enumerate() {
                let risk = usize::from_str(risk).expect("valid risk");
                assert_eq!(risk, board.risk_at((x, y)), "Position ({:?}, {:?})", x, y);
            }
        }
    }
}
