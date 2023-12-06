use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(input: &Vec<Vec<u16>>) -> u32 {
    let mut total_possibilities = 1;

    for i in 0..input[0].len() {
        let time = input[0][i];
        let distance = input[1][i];
        let mut possibilities = 0;

        for j in 0..time {
            if (time - j) * j > distance {
                possibilities += 1;
            }
        }

        if possibilities != 0 {
            total_possibilities *= possibilities;
        }
    }

    total_possibilities
}

fn part2(input: &Vec<Vec<u16>>) -> i32 {
    let time = input[0]
        .clone()
        .into_iter()
        .fold(String::new(), |acc, n| acc + &n.to_string())
        .parse::<u64>()
        .unwrap();

    let distance = input[1]
        .clone()
        .into_iter()
        .fold(String::new(), |acc, n| acc + &n.to_string())
        .parse::<u64>()
        .unwrap();

    let mut total_possibilities = 0;

    for i in 0..time {
        if (time - i) * i > distance {
            total_possibilities += 1;
        }
    }

    total_possibilities
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let regex = Regex::new(r#"\s+"#).unwrap();
    let input = reader
        .lines()
        .map(|line| {
            regex
                .replace_all(line.unwrap().split(':').nth(1).unwrap().trim_start(), " ")
                .split(' ')
                .map(|num| num.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
