use fancy_regex::Regex;
use rayon::prelude::*;
use std::fs;

fn main() {
    let input: Vec<u64> = fs::read_to_string("../input.txt")
        .unwrap()
        .split(',')
        .flat_map(|line| {
            let numbers = line.split_once('-').unwrap();
            numbers.0.parse().unwrap()..=numbers.1.parse().unwrap()
        })
        .collect();

    let regex = (
        Regex::new(r"^(\d+)\1$").unwrap(),
        Regex::new(r"^(\d+)\1+$").unwrap(),
    );

    let invalid = input
        .par_iter()
        .map(|number| {
            let number_str = number.to_string();
            (
                (regex.0.is_match(&number_str).unwrap() as u64) * number,
                (regex.1.is_match(&number_str).unwrap() as u64) * number,
            )
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}\n{}", invalid.0, invalid.1);
}
