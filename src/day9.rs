#![allow(dead_code)]
#![feature(map_first_last)]

use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let heat_map = load_file("./src/inputs/day9.txt");

    let low_points: Vec<Position> = heat_map
        .iter_positions()
        // find the ones with all neighbors at lower value
        .filter(|&(pos, val)| {
            heat_map
                .neighbors(pos)
                .all(|(_pos, neighbor_val)| neighbor_val > val)
        })
        .map(|(pos, _val)| pos)
        .collect();

    // Part 1
    let answer: isize = low_points
        .iter()
        // get risk score
        .filter_map(|&pos| heat_map.at(pos).map(|x| x + 1))
        .sum();

    println!("Day 9 Pt. 1 answer: {:?}", answer);

    // Part 2
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|pos| {
            let mut unchecked: BTreeSet<Position> = BTreeSet::from_iter([*pos]);
            let mut checked = BTreeSet::new();
            let mut in_basin = BTreeSet::new();

            while let Some(curr) = unchecked.pop_first() {
                in_basin.insert(curr);
                checked.insert(curr);

                for (next, value) in heat_map.neighbors(curr) {
                    match value {
                        9 => {}
                        _ => {
                            if !checked.contains(&next) {
                                unchecked.insert(next);
                            }
                        }
                    }
                }
            }

            in_basin.len()
        })
        .collect();
    basin_sizes.sort();

    println!(
        "Day 9 Pt 2 answer: {:?}",
        basin_sizes.iter().rev().take(3).product::<usize>()
    )
}

struct HeightMap(Vec<Vec<isize>>);

type Position = (isize, isize);

impl HeightMap {
    fn iter_positions(&self) -> impl Iterator<Item = (Position, isize)> + '_ {
        (0..self.width()).flat_map(move |x| {
            (0..self.height()).filter_map(move |y| {
                let pos = (x as isize, y as isize);
                self.at(pos).map(|val| (pos, val))
            })
        })
    }

    fn neighbors(&self, (x, y): Position) -> impl Iterator<Item = (Position, isize)> + '_ {
        [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
            .into_iter()
            .filter_map(|pos| self.at(pos).map(|val| (pos, val)))
    }

    fn at(&self, (x, y): Position) -> Option<isize> {
        if 0 <= x && x < (self.width() as isize) && 0 <= y && y < (self.height() as isize) {
            Some(self.0[y as usize][x as usize])
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

fn load_file(file_name: &str) -> HeightMap {
    let vals = read_to_string(file_name)
        .expect("valid file")
        .lines()
        .map(|line| {
            line.split("")
                .filter(|val| !val.is_empty())
                .map(|num| isize::from_str(num).expect("valid num"))
                .collect()
        })
        .collect();

    HeightMap(vals)
}
