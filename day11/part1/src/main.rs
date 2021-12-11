use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let mut input = raw_input.lines().map(|r| r.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut flashes = 0;

    for _ in 0..100 {

        let mut flashed = vec![];

        for r in 0..10 {
            for c in 0..10 {
                input[r][c] += 1;
            }
        }

        let mut new = true;
        while new {
            new = false;
            for r in 0..10 {
                for c in 0..10 {
                    let key = (r, c);
                    if !flashed.contains(&key) && input[r as usize][c as usize] > 9 {
                        new = true;
                        flashed.push(key);
                        for &(r2, c2) in get_adj(r, c).iter() {
                            input[r2][c2] += 1;
                        }
                    }
                }
            }
        }

        for r in 0..10 {
            for c in 0..10 {
                if input[r][c] > 9 {
                    input[r][c] = 0;
                }
            }
        }

        flashes += flashed.len();
    }

    println!("{}", flashes);
}

fn in_bounds(r: i32, c: i32) -> bool {
    return r >= 0 && r < 10 && c >= 0 && c < 10;
}

fn get_adj(r: i32, c: i32) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if in_bounds(r + dr, c + dc) {
                adj.push(((r + dr) as usize, (c + dc) as usize));
            }
        }
    }
    return adj;
}
