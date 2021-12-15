use std::fs::read_to_string;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let small_input: Vec<_> = raw_input.lines().map(|l| l.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>()).collect();

    let mut input: Vec<Vec<usize>> = vec![];

    for r in 0..500 {
        input.push(vec![]);
        for c in 0..500 {
            input[r].push(((small_input[r % 100][c % 100] + (((r as f32) / 100.).floor() as usize) + (((c as f32) / 100.).floor() as usize)) - 1) % 9 + 1);
        }
    }

    let mut finished: Vec<usize> = vec![];
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    queue.push_back((0, 0, 0));
    visited.insert((0, 0), 0);

    while !queue.is_empty() {
        let (r, c, v) = queue.pop_front().unwrap();

        if r == input.len() - 1 && c == input[0].len() - 1 {
            finished.push(v);
        }

        let adjs = get_adj(r as i32, c as i32);

        for (r2, c2) in adjs {
            let v2 = input[r2][c2];
            if !visited.contains_key(&(r2, c2)) || *visited.get(&(r2, c2)).unwrap() > (v + v2) {
                visited.insert((r2, c2), v + v2);
                queue.push_back((r2, c2, v + v2));
            }
        }
    }

    println!("{}", finished.iter().min().unwrap());
}

fn in_bounds(r: i32, c: i32) -> bool {
    return r >= 0 && r < 500 && c >= 0 && c < 500;
}

fn get_adj(r: i32, c: i32) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    if in_bounds(r + 1, c) {
        adj.push(((r + 1) as usize, c as usize));
    }
    if in_bounds(r - 1, c) {
        adj.push(((r - 1) as usize, c as usize));
    }
    if in_bounds(r, c + 1) {
        adj.push((r as usize, (c + 1) as usize));
    }
    if in_bounds(r, c - 1) {
        adj.push((r as usize, (c - 1) as usize));
    }
    return adj;
}
