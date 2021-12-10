use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines();

    let mut points: Vec<usize> = vec![0, 0, 0, 0];

    for line in input {
        let mut stack: Vec<char> = vec![];
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
                        match c {
                            ')' => points[0] += 1,
                            ']' => points[1] += 1,
                            '}' => points[2] += 1,
                            '>' => points[3] += 1,
                            _ => println!("Frick")
                        }
                        break;
                    }
                }
            }
        }
    }

    println!("{}", points[0] * 3 + points[1] * 57 + points[2] * 1197 + points[3] * 25137);
}
