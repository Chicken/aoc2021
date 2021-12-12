use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp;

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().map(|l| {
        let (start, end) = l.split_once(" -> ").unwrap();
        let (x1, y1) = start.split_once(",").unwrap();
        let (x2, y2) = end.split_once(",").unwrap();
        Line {
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap()
        }
    }).filter(|l| {
        l.x1 == l.x2 || l.y1 == l.y2
    });

    let mut counts = HashMap::new();

    for line in input {
        if line.x1 == line.x2 {
            for y in cmp::min(line.y1, line.y2)..=cmp::max(line.y1, line.y2) {
                let key = (line.x1, y);
                if counts.contains_key(&key) {
                    counts.insert(key, counts.get(&key).unwrap() + 1);
                } else {
                    counts.insert(key, 1);
                }
            }
        } else {
            for x in cmp::min(line.x1, line.x2)..=cmp::max(line.x1, line.x2) {
                let key = (x, line.y1);
                if counts.contains_key(&key) {
                    counts.insert(key, counts.get(&key).unwrap() + 1);
                } else {
                    counts.insert(key, 1);
                }
            }
        }
    }

    let mut sum = 0;
    for (_, count) in counts.iter() {
        if *count >= 2 {
            sum += 1;
        }
    }

    println!("{}", sum);
}
