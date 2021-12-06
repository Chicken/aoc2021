use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().next().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut groupped: Vec<usize> = vec![0,0,0,0,0,0,0,0,0];

    for f in input {
        groupped[f] += 1;
    }

    for _ in 0..256 {
        let mut new_gen: Vec<usize> = vec![0,0,0,0,0,0,0,0,0];
        for i in 0..groupped.len() {
            if i == 0 {
                new_gen[8] += groupped[i]; 
                new_gen[6] += groupped[i]; 
            } else {
                new_gen[i - 1] += groupped[i];
            }
        }
        groupped = new_gen;
    }

    println!("{}", groupped.iter().sum::<usize>());
}
