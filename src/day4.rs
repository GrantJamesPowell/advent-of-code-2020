// Cleaned up input using the following vim commands
// :2,$s/  \(\d\)/ 0\1/g
// :2,$s/^ \(\d\)/0\1/g

use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let (choosen_numbers, boards) = load_file("./src/inputs/day4.txt");

    // Part 1
    let (numbers, winner): (&[u64], &Board) = (5..)
        .map(|i| &choosen_numbers[..=i])
        .flat_map(|nums| boards.iter().map(move |board| (nums, board)))
        .find(|(nums, board)| board.are_numbers_winners_for_board(nums))
        .expect("at least one board wins");

    let sum_of_unmarked_numbers: u64 = winner.0.iter().filter(|num| !numbers.contains(num)).sum();

    println!(
        "Day 4 Pt 1 answer: {:?}",
        sum_of_unmarked_numbers * numbers.last().expect("numbers is not empty")
    );

    // Part 2
    let (numbers, last_winner) = boards
        .iter()
        .map(|board| {
            let winning_numbers = (0..)
                .map(|i| &choosen_numbers[..=i])
                .find(|nums| board.are_numbers_winners_for_board(nums))
                .expect("everyboard has a winning set");

            (winning_numbers, board)
        })
        .max_by_key(|(nums, _board)| nums.len())
        .expect("there is more than one board");

    let sum_of_unmarked_numbers: u64 = last_winner
        .0
        .iter()
        .filter(|num| !numbers.contains(num))
        .sum();

    println!(
        "Day 4 Pt 2 answer: {:?}",
        sum_of_unmarked_numbers * numbers.last().expect("numbers is not empty")
    );
}

#[derive(Debug)]
struct Board(Vec<u64>);

impl Board {
    fn are_numbers_winners_for_board(&self, nums: &[u64]) -> bool {
        self.groups()
            .any(|mut group| group.all(|num| nums.contains(&num)))
    }

    fn groups(&self) -> impl Iterator<Item = impl Iterator<Item = u64> + '_> + '_ {
        let mut groups = Vec::new();

        for row in 0..5 {
            groups.push(self.0[row * 5..][..5].iter().cloned().collect());
        }

        for col in 0..5 {
            let column = (0..5).map(|row_num| self.0[row_num * 5 + col]).collect();
            groups.push(column);
        }

        groups.into_iter().map(|group: Vec<u64>| group.into_iter())
    }
}

fn load_file(file_name: &str) -> (Vec<u64>, Vec<Board>) {
    let file = read_to_string(file_name).expect("can read file");
    let mut lines = file.lines();

    let chosen_numbers: Vec<u64> = lines
        .next()
        .expect("file isn't empty")
        .split(",")
        .map(|num| u64::from_str(num).expect("valid numbers in chosen_numbers"))
        .collect();

    let mut boards = Vec::new();

    while let Some("") = lines.next() {
        let board = (0..5)
            .flat_map(|_| {
                let line = lines.next().expect("is a complete board");
                line.split(" ")
                    .map(|num| u64::from_str(num).expect("valid num"))
            })
            .collect();
        boards.push(Board(board));
    }

    (chosen_numbers, boards)
}
