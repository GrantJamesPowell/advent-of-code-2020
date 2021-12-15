#![allow(dead_code)]

use std::collections::HashSet;
use std::{fmt::Debug, str::FromStr};

fn main() {
    let mut starting_board = load_file("./src/inputs/day11.txt");

    // Part 1
    let mut board = starting_board.clone();
    let answer = board.advance(100);
    println!("Day 11 Pt 1 answer: {:?}", answer);

    // Part 2
    let num_octs = starting_board.octopus_count();
    while num_octs != starting_board.advance_once() {}
    println!("Day 11 Pt 2 answer: {:?}", starting_board.turn);
}

type Position = (usize, usize);

#[derive(Clone)]
struct Board {
    board: Vec<Vec<Option<usize>>>,
    turn: usize,
}

impl Debug for Board {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "[")?;

        for row in self.board.iter() {
            for octopus in row.iter() {
                let c = match octopus {
                    Some(n) => n.to_string(),
                    None => "-".to_string(),
                };

                write!(fmt, "{}", c)?;
            }
            write!(fmt, "\n")?;
        }

        writeln!(fmt, "]")
    }
}

impl Board {
    fn octopus_count(&self) -> usize {
        self.board.iter().map(|row| row.len()).sum()
    }

    fn advance(&mut self, times: usize) -> usize {
        (0..times).map(|_| self.advance_once()).sum()
    }

    fn advance_once(&mut self) -> usize {
        self.turn += 1;
        // Increment all by one
        for row in self.board.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.as_mut().map(|val| *val += 1);
            }
        }

        let mut flashed = HashSet::new();
        // Resolve flashing
        while let Some(pos) = self.next_flashing() {
            flashed.insert(pos);

            for pos in self.neighbors(pos) {
                self.at_mut(pos).as_mut().map(|val| *val += 1);
            }

            *self.at_mut(pos) = None;
        }

        for pos in flashed.iter() {
            *self.at_mut(*pos) = Some(0);
        }

        flashed.len()
    }

    fn next_flashing(&self) -> Option<Position> {
        self.flashing().next()
    }

    fn flashing(&self) -> impl Iterator<Item = Position> + '_ {
        self.iter()
            .filter(|&(_, octopus)| match octopus {
                Some(num) if *num > 9 => true,
                _ => false,
            })
            .map(|(pos, _)| pos)
    }

    fn at(&self, (x, y): Position) -> &Option<usize> {
        &self.board[y][x]
    }

    fn at_mut(&mut self, (x, y): Position) -> &mut Option<usize> {
        &mut self.board[y][x]
    }

    fn iter(&self) -> impl Iterator<Item = (Position, &Option<usize>)> + '_ {
        (0..self.width())
            .flat_map(move |x| (0..self.height()).map(move |y| ((x, y), self.at((x, y)))))
    }

    fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        // pull these out so we don't capture `&self` in the closure
        let width = self.width();
        let height = self.height();

        let x = x as isize;
        let y = y as isize;

        [
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x - 1, y + 1),
            (x - 1, y - 1),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
        ]
        .into_iter()
        .filter(move |&(x, y)| 0 <= x && x < (width as isize) && 0 <= y && y < (height as isize))
        .map(|(x, y)| (x as usize, y as usize))
    }

    fn width(&self) -> usize {
        self.board[0].len()
    }

    fn height(&self) -> usize {
        self.board.len()
    }
}

fn load_file(file_name: &str) -> Board {
    let board = std::fs::read_to_string(file_name)
        .expect("file exists")
        .lines()
        .map(|line| {
            line.split("")
                .filter(|x| !x.is_empty())
                .map(|num| usize::from_str(num).expect("valid num"))
                .map(Some)
                .collect()
        })
        .collect();

    Board { board, turn: 0 }
}
