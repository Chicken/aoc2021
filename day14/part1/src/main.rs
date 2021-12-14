use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let (raw_template, raw_rules) = raw_input.split_once("\n\n").unwrap();
    let mut input = raw_template.clone().to_owned();
    let rules: Vec<_> = raw_rules.lines().map(|l| { l.split_once(" -> ").unwrap() }).collect();

    for _ in 0..10 {
        input = apply(&input, &rules);
    }

    println!("{}", most(&input) - least(&input));
}

fn apply(input: &str, rules: &Vec<(&str, &str)>) -> String {
    let mut new_str = input.clone().to_owned();
    for (rule, _) in rules.clone().iter() {
        let chars: Vec<_> = rule.chars().collect();
        let res = &format!("{}-{}", chars[0], chars[1]);
        while new_str.contains(rule) {
            new_str = new_str.replace(rule, res);
        }
    }
    for (rule, result) in rules.clone().iter() {
        let chars: Vec<_> = rule.chars().collect();
        let pattern = &format!("{}-{}", chars[0], chars[1]);
        let res = &format!("{}{}{}", chars[0], result, chars[1]);
        while new_str.contains(pattern) {
            new_str = new_str.replace(pattern, res);
        }
    }
    return new_str.to_string();
}

fn most(input: &str) -> usize {
    let mut most = 0;
    let mut chars =  input.chars().collect::<Vec<_>>();
    chars.dedup();
    for c in chars.iter() {
        let amount = input.matches(&c.to_string()).count();
        if amount > most {
            most = amount;
        }

    }
    return most;
}

fn least(input: &str) -> usize {
    let mut most = usize::MAX;
    let mut chars =  input.chars().collect::<Vec<_>>();
    chars.dedup();
    for c in chars.iter() {
        let amount = input.matches(&c.to_string()).count();
        if amount < most {
            most = amount;
        }

    }
    return most;
}
