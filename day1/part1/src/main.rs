use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input: Vec<u32> = raw_input.lines().map(|l| {
        return l.parse::<u32>().unwrap();
    }).collect();
    let mut total = 0;
    let mut prev = u32::MAX;
    for d in input {
        if d > prev {
            total += 1;
        }
        prev = d;
    }
    println!("{}", total);
}
