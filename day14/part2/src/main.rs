use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let raw_input = read_to_string("./input.txt").unwrap();
    let (raw_template, raw_rules) = raw_input.split_once("\n\n").unwrap();
    let input: Vec<_> = raw_template.chars().collect();
    let rule_pairs = raw_rules.lines().map(|l| { l.split_once(" -> ").unwrap() });

    let mut rules: HashMap<&str, &str> = HashMap::new();
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut chars: HashMap<char, usize> = HashMap::new();
    for (rule, result) in rule_pairs {
        rules.insert(rule, result);
    }
    for i in 0..(input.len() - 1) {
        let key = format!("{}{}", input[i], input[i + 1]);
        if !pairs.contains_key(&key) {
            pairs.insert(key, 1);
        } else {
            pairs.insert(key.clone(), pairs.get(&key).unwrap() + 1);
        }
    }
    for c in input.iter() {
        chars.insert(*c, or_zero(chars.get(c)) + 1);
    }

    for _ in 0..40 {
        for (pair, count) in pairs.clone() {
            let pp = pair.clone();
            let mut ab = pair.chars();
            let a = ab.next().unwrap().to_string();
            let b = ab.next().unwrap().to_string();
            let cs = rules.get(&*pair).unwrap();
            let c = cs.chars().next().unwrap();
            chars.insert(c, or_zero(chars.get(&c)) + 1);
            pairs.insert(pair, or_zero(pairs.get(&pp)) - count);
            let k1 = a + &c.to_string();
            pairs.insert(k1.clone(), or_zero(pairs.get(&k1)) + count);
            let k2 = c.to_string() + &b.to_string();
            pairs.insert(k2.clone(), or_zero(pairs.get(&k2)) + count);
        }
    }
    let mut chars: HashMap<char, usize> = HashMap::new();
    for (pair, amount) in pairs {
        let key = pair.chars().next().unwrap();
        chars.insert(key, match chars.get(&key) {
            Some(val) => val,
            None => &0usize
        } + amount);
    }
    let last = input.iter().last().unwrap();
    chars.insert(last.clone(), chars.get(&last).unwrap() + 1);

    let values = chars.values();
    println!("{}", values.clone().max().unwrap() - values.min().unwrap());
}

fn or_zero(opt: Option<&usize>) -> usize {
    match opt {
        Some(v) => v.to_owned(),
        None => 0usize,
    }
}
