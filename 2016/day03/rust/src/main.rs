use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let re = Regex::new(r"(\d+\s*){3}").unwrap();
    let input: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            re.find(line.unwrap().trim())
                .unwrap()
                .as_str()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut valid_part1 = 0;
    let mut valid_part2 = 0;

    for i in 0..input.len() {
        let mut triangle = input[i].clone();
        triangle.sort();
        if triangle[0] + triangle[1] > triangle[2] {
            valid_part1 += 1;
        }

        if i % 3 == 0 {
            for j in 0..3 {
                let mut triangle = vec![input[i][j], input[i + 1][j], input[i + 2][j]];
                triangle.sort();
                if triangle[0] + triangle[1] > triangle[2] {
                    valid_part2 += 1;
                }
            }
        }
    }

    println!("{valid_part1}\n{valid_part2}");
}
