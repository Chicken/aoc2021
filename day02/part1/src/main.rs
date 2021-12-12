use std::fs::read_to_string;

struct Inst {
    dir: String,
    amount: i32
}

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input: Vec<Inst> = raw_input.lines().map(|l| {
        let (dir, amount) = l.split_once(" ").unwrap();
        Inst {
            dir: dir.to_string(),
            amount: amount.parse().unwrap()
        }
    }).collect();
    
    let mut x = 0;
    let mut y = 0;

    for i in input {
        match i.dir.as_str() {
            "forward" => x += i.amount,
            "down" => y += i.amount,
            "up" => y -= i.amount,
            _ => println!("no")
        }
    }

    println!("{}", x * y);
}
