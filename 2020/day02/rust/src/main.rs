use regex::Regex;
use std::fs;

struct Policy {
    min: usize,
    max: usize,
    char: char,
}

fn main() {
    let input: Vec<(Policy, String)> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
            let captures = re.captures(line).unwrap();

            (
                Policy {
                    min: captures[1].parse().unwrap(),
                    max: captures[2].parse().unwrap(),
                    char: captures[3].chars().next().unwrap(),
                },
                captures[4].to_string(),
            )
        })
        .collect();

    let mut valid: (u32, u32) = (0, 0);

    for (policy, password) in &input {
        let char_count = password.chars().filter(|&c| c == policy.char).count();
        if char_count >= policy.min && char_count <= policy.max {
            valid.0 += 1;
        }

        if (password.chars().nth(policy.min - 1) == Some(policy.char))
            ^ (password.chars().nth(policy.max - 1) == Some(policy.char))
        {
            valid.1 += 1;
        }
    }

    println!("{}\n{}", valid.0, valid.1);
}
