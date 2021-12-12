use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let mut input = raw_input.lines().next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    for _ in 0..80 {
        for i in 0..input.len() {
            if input[i] == 0 {
                input.insert(input.len(), 8);
                input[i] = 6;
            } else {
                input[i] -= 1;
            }
        }
    }

    println!("{}", input.len());
}
