#![allow(dead_code)]

use std::{collections::HashMap, iter::once, str::FromStr};

fn main() {
    let board = load_file("./src/inputs/day15.txt");

    // Day 1
    let answer: usize = find_paths(&[], (0, 0), &board, &mut HashMap::new())
        .into_iter()
        .map(|path| path.iter().map(|&pos| board.risk_at(pos)).sum())
        .min()
        .expect("at least one item");

    println!("Day 15 Pt. 1 answer: {:?}", answer);
}

fn find_paths(
    previous_path: &[Position],
    current: Position,
    board: &Board,
    memoization: &mut HashMap<Position, Vec<Vec<Position>>>,
) -> Vec<Vec<Position>> {
    if current == board.goal() {
        return vec![];
    }

    if let Some(paths) = memoization.get(&current) {
        return paths.clone();
    }

    let mut paths = Vec::new();

    board
        .neighbors(current)
        .filter(|neighbor| !previous_path.contains(neighbor))
        .for_each(|neighbor| {
            println!(" -> {:?}", neighbor);
            let new_path = previous_path
                .iter()
                .chain(once(&neighbor))
                .cloned()
                .collect::<Vec<_>>();

            let mut paths_from_neighbor = find_paths(&new_path, neighbor, board, memoization);
            memoization.insert(neighbor, paths.clone());

            paths_from_neighbor
                .iter_mut()
                .for_each(|path| path.insert(0, neighbor));

            paths.extend(paths_from_neighbor)
        });

    paths
}

type Position = (usize, usize);

struct Board(Vec<Vec<usize>>);

impl Board {
    fn risk_at(&self, (x, y): Position) -> usize {
        self.0[y][x]
    }

    fn goal(&self) -> Position {
        (self.width() - 1, self.height() - 1)
    }

    fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let x = x as isize;
        let y = y as isize;

        let width = self.width() as isize;
        let height = self.height() as isize;

        [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
            .into_iter()
            .filter(move |&(x, y)| 0 <= x && x < width && 0 <= y && y < height)
            .map(|(x, y)| (x as usize, y as usize))
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
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
