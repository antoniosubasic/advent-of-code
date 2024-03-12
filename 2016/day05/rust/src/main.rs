use std::fs;
use md5;
use hex;

fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut j = 0;

    while password.len() != 8 {
        let mut hash = String::new();

        while !hash.starts_with("0".repeat(5).as_str()) {
            let digest = md5::compute(format!("{}{}", input, j));
            hash = hex::encode(digest.as_ref());
            j += 1;
        }

        password.push(hash.chars().nth(5).unwrap());
    }

    password
}

fn part2(input: &str) -> String {
    let mut password = ['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut j: u32 = 0;

    while password.contains(&'_') {
        let mut hash = String::new();

        while !hash.starts_with("0".repeat(5).as_str()) {
            let digest = md5::compute(format!("{}{}", input, j));
            hash = hex::encode(digest.as_ref());
            j += 1;
        }

        let pos = (hash.chars().nth(5).unwrap() as usize) - ('0' as usize);

        if pos < 8 && password[pos] == '_' {
            password[pos] = hash.chars().nth(6).unwrap();
        }
    }

    password.iter().collect()
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
