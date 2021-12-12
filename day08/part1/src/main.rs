use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().map(|x| {
        let (_, num) = x.split_once(" | ").unwrap();
        return num.split(" ").collect::<Vec<_>>();
    }).collect::<Vec<Vec<_>>>();

    let mut sum = 0;
    for line in input.iter() {
        for number in line.iter() {
            if number.len() == 2
            || number.len() == 4
            || number.len() == 3
            || number.len() == 7 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
