use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap());

    let mut smallest = i32::MAX;
    let min = input.clone().min().unwrap();
    let max = input.clone().max().unwrap();

    // haha caveman go brr
    for x in min..max {
        let mut fuel = 0;
        for n in input.clone() {
            let steps = (n - x).abs();
            // haha caveman go brr
            // for i in 1..=steps {
            //     fuel += i;
            // }
            // replaced with a faster solution for github xd
            fuel += (steps  * (steps + 1)) / 2;
        }
        if fuel < smallest {
            smallest = fuel;
        }
    }

    println!("{}", smallest);
}
