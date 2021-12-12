use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines();

    let mut common: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for l in input {
        for (i, c) in l.chars().enumerate() {
            if c == "1".chars().nth(0).unwrap() {
                common[i] += 1;
            }
        }
    }

    let gamma: String = common.iter().map(|n| {
        return if *n > 500 {
            "1"
        } else {
            "0"
        }
    }).collect::<Vec<&str>>().join("");

    let epsilon = gamma.replace("1", "2").replace("0", "1").replace("2", "0");

    println!("{}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap());
}
