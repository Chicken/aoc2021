use std::fs::read_to_string;

fn travel(cave: &str, input: &Vec<(&str, &str)>, visited: &Vec<&str>) -> usize {
    let mut sum = 0;
    for (from, to) in input.iter() {
        if *from == cave {
            if *to == "end" {
                sum += 1;
            } else if &to.to_lowercase() == to {
                if !visited.contains(to) {
                    let mut vis = visited.clone();
                    vis.push(to);
                    sum += travel(to, input, &vis);
                }
            } else {
                sum += travel(to, input, visited);
            }
        }
    }
    return sum;
}

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().map(|l| {
        l.split_once("-").unwrap()
    }).collect::<Vec<_>>();
    let paths: Vec<(&str, &str)> = input.clone().into_iter().chain(input.clone().into_iter().map(|(from, to)| (to, from))).collect();

    let visited: Vec<&str> = vec!["start"];
    println!("{}", travel("start", &paths, &visited));
}
