use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap());

    let mut smallest = i32::MAX;
    let min = input.clone().min().unwrap();
    let max = input.clone().max().unwrap();

    for x in min..max {
        let mut fuel = 0;
        for n in input.clone() {
            fuel +=  (n - x).abs();
        }
        if fuel < smallest {
            smallest = fuel;
        }
    }

    println!("{}", smallest);
}
