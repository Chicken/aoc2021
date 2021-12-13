use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let (raw_points, raw_folds) = raw_input.split_once("\n\n").unwrap();
    let mut points: Vec<(i32, i32)> = raw_points.lines().map(|l| {
        let (x, y) = l.split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect();
    let folds: Vec<(&str, i32)> = raw_folds.lines().map(|l| {
        let (_, raw) = l.split_once("g ").unwrap();
        let (dir, coord) = raw.split_once("=").unwrap();
        (dir, coord.parse().unwrap())
    }).collect();

    for f in folds {
        points = fold(&points, &f);
    }

    let (highest_x,_) = points.iter().reduce(|a, c| {
        if a.0 < c.0 { c } else { a }
    }).unwrap();

    let (_, highest_y) = points.iter().reduce(|a, c| {
        if a.1 < c.1 { c } else { a }
    }).unwrap();

    for y in 0i32..=*highest_y {
        for x in 0i32..=*highest_x {
            let val = points.iter().find(|(px, py)| {
                *px == x && *py == y
            });
            match val {
                Some(_) => print!("#"),
                None => print!(".")
            }
        }
        println!();
    }
}

fn fold(points: &Vec<(i32, i32)>, (dir, coord): &(&str, i32)) -> Vec<(i32, i32)> {
    let mut new_points: Vec<(i32, i32)> = vec![];
    if *dir == "x" {
        for key in points.iter() {
            let (px, py) = key;
            if px > coord {
                let new_key: (i32, i32) = (px - 2 * (px - coord), py.clone());
                if !new_points.contains(&new_key) {
                    new_points.push(new_key);
                }
            } else {
                if !new_points.contains(key) {
                    new_points.push(*key);
                }
            }
        }
    } else {
        for key in points.iter() {
            let (px, py) = key;
            if py > coord {
                let new_key: (i32, i32) = (px.clone(), py  - 2 * (py - coord));
                if !new_points.contains(&new_key) {
                    new_points.push(new_key);
                }
            } else {
                if !new_points.contains(key) {
                    new_points.push(*key);
                }
            }
        }
    }
    return new_points.to_owned();
}
