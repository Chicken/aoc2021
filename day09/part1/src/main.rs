use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().map(|l| l.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sum = 0;

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            let val = input[r][c];
            if (r == input.len() - 1 || val < input[r + 1][c])
            && (r == 0 || val < input[r - 1][c])
            && (c == input[r].len() - 1 || val < input[r][c + 1])
            && (c == 0 || val < input[r][c - 1])
            && ((r == input.len() - 1 || c == input[r].len() - 1) || val < input[r + 1][c + 1])
            && ((r == input.len() - 1 || c == 0 ) || val < input[r + 1][c - 1])
            && ((r == 0 || c == input[r].len() - 1) || val < input[r - 1][c + 1])
            && ((r == 0 || c == 0) || val < input[r - 1][c - 1]) {
                sum += val + 1;
            }
        }
    }

    println!("{}", sum);
}
