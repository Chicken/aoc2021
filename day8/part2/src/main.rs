use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let input = raw_input.lines().map(|x| {
        let (unique, num) = x.split_once(" | ").unwrap();
        return (
            unique.split(" ").collect::<Vec<_>>(),
            num.split(" ").collect::<Vec<_>>()
        );
    }).collect::<Vec<_>>();

    let mut sum = 0;
    for (unique, num) in input.iter() {
        sum += solve(unique, num);
    }
    println!("{}", sum);
}

fn sortstr(letters: &str) -> String {
    let mut chars = letters.chars().collect::<Vec<_>>();
    chars.sort_by(|a, b| a.cmp(b));
    return String::from_iter(chars);
}

fn solve(unique: &Vec<&str>, num: &Vec<&str>) -> u32 { 
    // Btw this is a horrible solution and I hate it
     
    let digit_one = unique.iter().find(|x| x.len() == 2).unwrap();
    let digit_four = unique.iter().find(|x| x.len() == 4).unwrap();
    let digit_seven = unique.iter().find(|x| x.len() == 3).unwrap();
    let digit_eight = unique.iter().find(|x| x.len() == 7).unwrap();

    let top_char = digit_seven.chars()
        .find(|x| !digit_one.contains(*x))
        .unwrap();
    let middle_char = digit_four.chars()
        .filter(|x| !digit_one.contains(*x))
        .find(|x| unique.iter().filter(|x2| x2.contains(*x)).collect::<Vec<_>>().len() == 7)
        .unwrap();
    let bottom_char = unique.iter()
        .find(|x| x.len() == 5 && x.contains(top_char) && x.contains(middle_char) && digit_one.chars().all(|c| x.contains(c)))
        .unwrap()
        .chars()
        .find(|&x| x != top_char && x != middle_char && digit_one.chars().all(|c| x != c))
        .unwrap();

    let left_chars = digit_eight.chars()
        .filter(|&x| x != top_char && x != middle_char && x != bottom_char && digit_one.chars().all(|c| x != c))
        .collect::<Vec<_>>();

    let left_top_char = digit_four.chars()
        .find(|&x| x != middle_char && digit_one.chars().all(|c| x != c))
        .unwrap();

    let left_bottom_char = *left_chars.iter().find(|&&x| x != left_top_char).unwrap();

    let right_top_char = unique.iter()
        .find(|x| x.len() == 5 && x.contains(left_bottom_char))
        .unwrap()
        .chars()
        .find(|&x| x != top_char && x != middle_char && x != bottom_char && x != left_bottom_char)
        .unwrap();

    let right_bottom_char = digit_one.chars().find(|&x| x != right_top_char).unwrap();

    let zero = sortstr(&format!("{}{}{}{}{}{}", top_char, bottom_char, left_top_char, left_bottom_char, right_bottom_char, right_top_char));
    let one = sortstr(&format!("{}{}", right_top_char, right_bottom_char));
    let two = sortstr(&format!("{}{}{}{}{}", top_char, middle_char, bottom_char, right_top_char, left_bottom_char));
    let three = sortstr(&format!("{}{}{}{}{}", top_char, middle_char, bottom_char, right_top_char, right_bottom_char));
    let four = sortstr(&format!("{}{}{}{}", left_top_char, middle_char, right_top_char, right_bottom_char));
    let five = sortstr(&format!("{}{}{}{}{}", top_char, middle_char, bottom_char, left_top_char, right_bottom_char));
    let six = sortstr(&format!("{}{}{}{}{}{}", top_char, middle_char, bottom_char, left_top_char, left_bottom_char, right_bottom_char));
    let seven = sortstr(&format!("{}{}{}", top_char, right_top_char, right_bottom_char));
    let eight = sortstr(&format!("{}{}{}{}{}{}{}", top_char, middle_char, bottom_char, left_top_char, left_bottom_char, right_bottom_char, right_top_char));
    let nine = sortstr(&format!("{}{}{}{}{}{}", top_char, middle_char, bottom_char, left_top_char, right_bottom_char, right_top_char));

    let mut sum = 0;
    for (i,n) in num.iter().rev().enumerate() {
        let nn = sortstr(n);
        if nn == zero {
            sum += 0 * (10u32).pow(i as u32)
        } else if nn == one {
            sum += 1 * (10u32).pow(i as u32)
        } else if nn == two {
            sum += 2 * (10u32).pow(i as u32)
        } else if nn == three {
            sum += 3 * (10u32).pow(i as u32)
        } else if nn == four {
            sum += 4 * (10u32).pow(i as u32)
        } else if nn == five {
            sum += 5 * (10u32).pow(i as u32)
        } else if nn == six {
            sum += 6 * (10u32).pow(i as u32)
        } else if nn == seven {
            sum += 7 * (10u32).pow(i as u32)
        } else if nn == eight {
            sum += 8 * (10u32).pow(i as u32)
        } else if nn == nine {
            sum += 9 * (10u32).pow(i as u32)
        }
    }

    return sum;
}
