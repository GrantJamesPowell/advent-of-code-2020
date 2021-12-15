use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let inputs = load_file("./src/inputs/day6.txt");

    // Part 1

    let mut fish_counts = FishCounts::from_iter(inputs.clone());
    fish_counts.advance(80);
    println!("Day 6 pt 1 answer: {:?}", fish_counts.total());

    // Part 2

    let mut fish_counts = FishCounts::from_iter(inputs.clone());
    fish_counts.advance(256);
    println!("Day 6 pt 2 answer: {:?}", fish_counts.total());
}

#[derive(Debug, PartialEq, Eq)]
struct FishCounts([usize; 9]);

impl FishCounts {
    fn total(&self) -> usize {
        self.0.iter().sum()
    }

    fn advance_once(&mut self) {
        let [day0, day1, day2, day3, day4, day5, day6, day7, day8] = self.0;

        self.0 = [day1, day2, day3, day4, day5, day6, day0 + day7, day8, day0];
    }

    fn advance(&mut self, times: usize) {
        for _ in 0..times {
            self.advance_once()
        }
    }
}

impl FromIterator<usize> for FishCounts {
    fn from_iter<T: IntoIterator<Item = usize>>(t: T) -> Self {
        let mut fish_counts = FishCounts([0; 9]);

        for fish in t {
            fish_counts.0[fish] += 1;
        }

        fish_counts
    }
}

fn load_file(file_name: &str) -> Vec<usize> {
    read_to_string(file_name)
        .expect("can read file")
        .lines()
        .next()
        .expect("only one line")
        .split(",")
        .map(|num| usize::from_str(num).expect("valid number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advancing() {
        let start = vec![3, 4, 3, 1, 2];
        let sim = [
            vec![2, 3, 2, 0, 1],
            vec![1, 2, 1, 6, 0, 8],
            vec![0, 1, 0, 5, 6, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 7, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 7, 7, 8],
            vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 7],
            vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6],
            vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5],
            vec![1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 8],
            vec![0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 7, 8],
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 7, 8, 8, 8],
            vec![5, 6, 5, 3, 4, 5, 6, 0, 0, 1, 5, 6, 7, 7, 7, 8, 8],
            vec![4, 5, 4, 2, 3, 4, 5, 6, 6, 0, 4, 5, 6, 6, 6, 7, 7, 8, 8],
            vec![3, 4, 3, 1, 2, 3, 4, 5, 5, 6, 3, 4, 5, 5, 5, 6, 6, 7, 7, 8],
            vec![2, 3, 2, 0, 1, 2, 3, 4, 4, 5, 2, 3, 4, 4, 4, 5, 5, 6, 6, 7],
            vec![
                1, 2, 1, 6, 0, 1, 2, 3, 3, 4, 1, 2, 3, 3, 3, 4, 4, 5, 5, 6, 8,
            ],
            vec![
                0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8,
            ],
            vec![
                6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
            ],
        ];

        // Test specifics
        let mut counts = FishCounts::from_iter(start.clone());

        for expected in sim {
            counts.advance_once();
            assert_eq!(counts, FishCounts::from_iter(expected));
        }

        assert_eq!(counts.total(), 26);

        // test after 80 days
        let mut counts = FishCounts::from_iter(start);

        counts.advance(80);
        assert_eq!(counts.total(), 5934);
    }
}
