use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn part1(input: &Vec<String>) -> i32 {
    0
}

fn part2(input: &Vec<String>) -> i32 {
    0
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
