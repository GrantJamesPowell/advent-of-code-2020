fn main() {
    let file = std::fs::read_to_string("./src/inputs/day10.txt").expect("file exists");
    let inputs: Vec<&str> = file.lines().collect();

    // Part 1
    let answer: u64 = inputs
        .iter()
        .filter_map(|line| character_stack_or_invalid_char(line.chars()).err())
        .map(|invalid| match invalid {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            other => panic!("Invalid char, {:?}", other),
        })
        .sum();

    println!("Day 10 Pt 1: {:?}", answer);

    // Part2
    let mut scores: Vec<usize> = inputs
        .iter()
        .filter_map(|line| character_stack_or_invalid_char(line.chars()).ok())
        .map(|stack| {
            stack.into_iter().rev().map(|opener| match opener {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => panic!("invalid char"),
            })
        })
        .map(|closers| {
            closers.fold(0, |score, closer| {
                (score * 5)
                    + match closer {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("invalid char"),
                    }
            })
        })
        .collect();

    scores.sort();
    println!("Day 10 Pt 2: {:?}", scores[scores.len() / 2]);
}

fn character_stack_or_invalid_char(inputs: impl Iterator<Item = char>) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();

    for input in inputs {
        match input {
            '(' | '{' | '[' | '<' => stack.push(input),
            ')' | '}' | ']' | '>' => {
                if let Some(opener) = stack.pop() {
                    match (opener, input) {
                        ('(', ')') | ('{', '}') | ('[', ']') | ('<', '>') => continue,
                        _ => return Err(input),
                    }
                } else {
                    return Err(input);
                }
            }
            _ => panic!("invalid char: {:?}", input),
        }
    }

    Ok(stack)
}
