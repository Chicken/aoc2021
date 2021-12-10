use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines();

    let mut points: Vec<usize> = vec![];

    for line in input {
        let mut stack: Vec<char> = vec![];
        let mut invalid = false;
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),
                _ => {
                    let counter = match c {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        '>' => '<',
                        _=> ' ',
                    };
                    if stack.len() == 0 || stack.pop().unwrap() != counter {
                        invalid = true;
                        break;
                    }
                }
            }
        }
        if !invalid {
            stack.reverse();
            let mut score = 0;
            for c in stack {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };
            }
            points.push(score);
        }
    };

    points.sort();
    println!("{}", points[points.len()  / 2]);
}
