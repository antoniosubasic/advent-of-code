use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<i32> = reader
        .lines()
        .map(|line| {
            let mut line = line.unwrap();
            let operand = line.chars().next().unwrap();
            line.remove(0);
            line.parse::<i32>().unwrap() * if operand == '-' { -1 } else { 1 }
        })
        .collect();

    let mut i = 0;
    let mut current_frequency = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();

    while !frequencies.contains(&current_frequency) {
        frequencies.insert(current_frequency);
        current_frequency += input[i % input.len()];
        i += 1;
    }

    println!("{}\n{}", input.iter().sum::<i32>(), current_frequency);
}
