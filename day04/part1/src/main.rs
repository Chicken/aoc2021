use std::fs::read_to_string;

fn check_board(board: &Vec<Vec<i32>>, numbers: &Vec<i32>) -> bool {
    for r in board {
        let mut sum = 0;
        for c in r {
            if numbers.contains(c) {
                sum += 1;
            }
        }
        if sum == 5 {
            return true;
        }
    }
    for c in 0..5 {
        let mut sum = 0;
        for r in 0..5 {
            if numbers.contains(&board[r][c]) {
                sum += 1;
            }
        }
        if sum == 5 {
            return true;
        }
    }
    return false;
}

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let mut input = raw_input.trim().split("\n\n");
    let numbers_vec = input.next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let numbers = numbers_vec.iter();
    let boards_str = input.collect::<Vec<_>>();
    let boards_vec = boards_str.iter().map(|b| {
        b.split("\n").map(|r| {
            r.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()
        }).collect::<Vec<Vec<_>>>()
    }).collect::<Vec<Vec<Vec<_>>>>();
    let boards = boards_vec.iter();

    let mut checked: Vec<i32> = vec![]; 

    for n in numbers.clone() {
        checked.push(*n);
        for board in boards.clone() {
            if check_board(&board, &checked) {
                let mut sum = 0;
                for r in board.iter() {
                    for c in r.iter() {
                        if !checked.contains(c) {
                            sum += c;
                        } 
                    }
                }
                println!("{}", sum * n);
                std::process::exit(0);
            }
        }
    }
}
