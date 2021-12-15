use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./src/inputs/day8.txt").expect("file exists");
    let puzzles = parse(&file);

    // Part 1
    let occurances_of_1_4_7_8: usize = puzzles
        .iter()
        .flat_map(|puzzle| puzzle.outputs.iter())
        .filter(|signal| matches!(signal.len(), 2 | 3 | 4 | 7))
        .count();

    println!("Day 8 Pt 1 answer: {:?}", occurances_of_1_4_7_8);

    // Part 2
    //
    let sum_of_outputs: usize = puzzles.iter().map(|puzzle| puzzle.solve()).sum();
    println!("Day 8 Pt 2 answer: {:?}", sum_of_outputs)
}

#[derive(Debug)]
struct Puzzle<'a> {
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn solve(&self) -> usize {
        let one = *self
            .inputs
            .iter()
            .find(|input| input.len() == 2)
            .expect("contains a one");
        let four = *self
            .inputs
            .iter()
            .find(|input| input.len() == 4)
            .expect("contains a four");
        let seven = *self
            .inputs
            .iter()
            .find(|input| input.len() == 3)
            .expect("contains a seven");
        let eight = *self
            .inputs
            .iter()
            .find(|input| input.len() == 7)
            .expect("contains a eight");
        let zero = find_num(&self.inputs, [(2, one), (3, seven), (3, four), (6, eight)]);
        let two = find_num(
            &self.inputs,
            [(4, zero), (1, one), (2, four), (2, seven), (5, eight)],
        );
        let three = find_num(
            &self.inputs,
            [
                (4, zero),
                (2, one),
                (4, two),
                (3, four),
                (3, seven),
                (5, eight),
            ],
        );
        let five = find_num(
            &self.inputs,
            [
                (4, zero),
                (1, one),
                (3, two),
                (4, three),
                (2, seven),
                (5, eight),
            ],
        );
        let six = find_num(
            &self.inputs,
            [
                (5, zero),
                (1, one),
                (4, two),
                (4, three),
                (3, four),
                (5, five),
            ],
        );

        let nine = find_num(
            &self.inputs,
            [
                (5, zero),
                (2, one),
                (4, two),
                (5, three),
                (4, four),
                (5, five),
                (5, six),
                (3, seven),
                (6, eight),
            ],
        );

        let cases: Vec<(usize, Vec<char>)> =
            [zero, one, two, three, four, five, six, seven, eight, nine]
                .iter()
                .map(|case| {
                    let mut chars: Vec<char> = case.chars().collect();
                    chars.sort();
                    chars
                })
                .enumerate()
                .collect();

        self.outputs
            .iter()
            .rev()
            .map(|&output| {
                let mut output_chars: Vec<char> = output.chars().collect();
                output_chars.sort();
                cases
                    .iter()
                    .find(|(_num, case)| case == &output_chars)
                    .map(|(num, _)| num)
                    .expect("found one")
            })
            .enumerate()
            .map(|(idx, value)| value * (10_usize.pow(idx as u32)))
            .sum()
    }
}

fn find_num<'a, 'b>(
    inputs: &[&'a str],
    cases: impl IntoIterator<Item = (usize, &'b str)>,
) -> &'a str {
    let cases: Vec<_> = cases.into_iter().collect();
    let mut matching: Vec<&str> = inputs
        .iter()
        .filter(|input| matches_pattern(input, cases.iter().cloned()))
        .cloned()
        .collect();

    assert_eq!(matching.len(), 1);
    matching.pop().expect("we know there is one")
}

fn matches_pattern<'a>(input: &str, cases: impl IntoIterator<Item = (usize, &'a str)>) -> bool {
    let mut cases = cases.into_iter();
    cases.all(|(expected_number, pattern)| {
        input.chars().filter(|c| pattern.contains(*c)).count() == expected_number
    })
}

fn parse(file: &str) -> Vec<Puzzle<'_>> {
    file.lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let inputs = parts.next().expect("has inputs").split(" ").collect();
            let outputs = parts.next().expect("has outputs").split(" ").collect();

            Puzzle { inputs, outputs }
        })
        .collect()
}
