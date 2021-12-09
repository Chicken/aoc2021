use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().map(|l| l.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sums: Vec<usize> = vec![];

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
                let mut found = vec![(r, c)];
                sums.push(find_basin(&input, r, c, &mut found).len());
            }
        }
    }

    sums.sort();
    sums.reverse();
    println!("{}", sums[0] * sums[1] * sums[2]);
}

fn find_basin(input: &Vec<Vec<usize>>, r: usize, c: usize, found: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let val = input[r][c];
    if r < input.len() - 1 && val < input[r + 1][c] && input[r + 1][c] != 9 && !found.contains(&(r + 1, c)) {
        found.push((r + 1, c));
        find_basin(input, r + 1, c, found);
    }
    if r > 0  && val < input[r - 1][c] && input[r - 1][c] != 9 && !found.contains(&(r - 1, c)) {
        found.push((r - 1, c));
        find_basin(input, r - 1, c, found);
    }
    if c < input[r].len() - 1  && val < input[r][c + 1] && input[r][c + 1] != 9 && !found.contains(&(r, c + 1)) {
        found.push((r, c + 1));
        find_basin(input, r, c + 1, found);
    }
    if c > 0  && val < input[r][c - 1] && input[r][c - 1] != 9 && !found.contains(&(r, c - 1)) {
        found.push((r, c - 1));
        find_basin(input, r, c - 1, found);
    }
    return found.to_vec();
}
