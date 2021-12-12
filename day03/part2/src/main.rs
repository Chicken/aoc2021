use std::fs::read_to_string;

fn get_num(input: &Vec<&str>, b: bool) -> isize {
    let mut arr = input.clone();
    let mut i = 0;
    while arr.len() > 1 {
        let c: f32 = arr.clone().iter().fold(0 as f32, |t, a| if a.chars().nth(i).unwrap() == '1' { t + 1. } else { t });
        if (c >= f32::ceil((arr.len() as f32) / 2.)) == b {
            arr = arr.clone().iter().filter(|x| x.chars().nth(i).unwrap() == '1').collect::<Vec<_>>().iter().map(|&&x| x).collect::<Vec<&str>>();
        } else {
            arr = arr.clone().iter().filter(|x| x.chars().nth(i).unwrap() == '0').collect::<Vec<_>>().iter().map(|&&x| x).collect::<Vec<&str>>();
        }
        i += 1;
    }
    isize::from_str_radix(&arr[0], 2).unwrap()
}

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = raw_input.lines().collect();

    println!("{}", get_num(&lines, true) * get_num(&lines, false));
}
