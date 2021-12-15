#![allow(dead_code)]

use std::str::FromStr;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let line_segments = load_file("./src/inputs/day5.txt");

    // Part 1
    let mut point_counts = HashMap::new();

    line_segments
        .iter()
        .filter(|segment| matches!(segment.direction(), Vertical | Horizontal))
        .flat_map(|segment| segment.points())
        .for_each(|point| {
            let count = point_counts.entry(point).or_insert(0);
            *count += 1;
        });

    let answer = point_counts.into_iter().filter(|&(_k, v)| v > 1).count();

    println!("Day 5 pt 1 answer: {:?}", answer);

    // Part 2
    let mut point_counts = HashMap::new();

    line_segments
        .iter()
        .flat_map(|segment| segment.points())
        .for_each(|point| {
            let count = point_counts.entry(point).or_insert(0);
            *count += 1;
        });

    let answer = point_counts.into_iter().filter(|&(_k, v)| v > 1).count();

    println!("Day 5 pt 2 answer: {:?}", answer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Horizontal,
    Vertical,
    UpDiagonal,
    DownDiagonal,
}

use Direction::*;

impl LineSegment {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let x1 = self.start.x.min(self.end.x);
        let x2 = self.start.x.max(self.end.x);

        let y1 = self.start.y.min(self.end.y);
        let y2 = self.start.y.max(self.end.y);

        match self.direction() {
            Horizontal => (x1..=x2)
                .map(|x| Point { x, y: y1 })
                .collect::<Vec<_>>()
                .into_iter(),
            Vertical => (y1..=y2)
                .map(|y| Point { x: x1, y })
                .collect::<Vec<_>>()
                .into_iter(),
            UpDiagonal => (x1..=x2)
                .zip(y1..=y2)
                .map(|(x, y)| Point { x, y })
                .collect::<Vec<_>>()
                .into_iter(),
            DownDiagonal => (x1..=x2)
                .zip((y1..=y2).rev())
                .map(|(x, y)| Point { x, y })
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }

    fn direction(&self) -> Direction {
        use std::cmp::Ordering::*;

        let x_compare = self.start.x.cmp(&self.end.x);
        let y_compare = self.start.y.cmp(&self.end.y);

        match (x_compare, y_compare) {
            (Equal, _) => Vertical,
            (_, Equal) => Horizontal,
            (Greater, Greater) => UpDiagonal,
            (Less, Less) => UpDiagonal,
            (Less, Greater) => DownDiagonal,
            (Greater, Less) => DownDiagonal,
        }
    }

    fn slope(&self) -> f64 {
        (self.end.y - self.start.y) as f64 / (self.end.x - self.start.x) as f64
    }

    fn y_intercept(&self) -> f64 {
        (self.start.y as f64) - (self.slope() * self.start.x as f64)
    }
}

fn load_file(file_name: &str) -> Vec<LineSegment> {
    let file = read_to_string(file_name).expect("can read file");

    file.lines()
        .map(|line| {
            line.split(" -> ").map(|point| {
                let mut components = point
                    .split(",")
                    .map(|num| i64::from_str(num).expect("valid number"));

                let x = components.next().expect("has an x");
                let y = components.next().expect("has a y");

                Point { x, y }
            })
        })
        .map(|mut points| {
            let start = points.next().expect("has a start");
            let end = points.next().expect("has a end");

            LineSegment { start, end }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonal_line_yields_correct_points() {
        let line = LineSegment {
            start: Point { x: 1, y: 1 },
            end: Point { x: 3, y: 3 },
        };

        assert_eq!(line.direction(), UpDiagonal);
        let expected = [(1, 1), (2, 2), (3, 3)]
            .iter()
            .map(|&(x, y)| Point { x, y });
        assert!(line.points().eq(expected));

        let line = LineSegment {
            start: Point { x: 3, y: 3 },
            end: Point { x: 1, y: 1 },
        };

        assert_eq!(line.direction(), UpDiagonal);
        let expected = [(1, 1), (2, 2), (3, 3)]
            .iter()
            .map(|&(x, y)| Point { x, y });
        assert!(line.points().eq(expected));

        let line = LineSegment {
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 9 },
        };

        assert_eq!(line.direction(), DownDiagonal);
        let expected = [(7, 9), (8, 8), (9, 7)]
            .iter()
            .map(|&(x, y)| Point { x, y })
            .collect::<Vec<_>>();
        assert_eq!(line.points().collect::<Vec<_>>(), expected);

        let line = LineSegment {
            start: Point { x: 7, y: 9 },
            end: Point { x: 9, y: 7 },
        };

        assert_eq!(line.direction(), DownDiagonal);
        let expected = [(7, 9), (8, 8), (9, 7)]
            .iter()
            .map(|&(x, y)| Point { x, y })
            .collect::<Vec<_>>();
        assert_eq!(line.points().collect::<Vec<_>>(), expected);
    }
}
