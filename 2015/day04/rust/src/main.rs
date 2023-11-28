use std::fs;
use md5;
use hex;

fn part(input: &str, starting_zeros: usize) -> i32 {
    let mut i = 0;
    let mut hash = String::new();

    while !hash.starts_with("0".repeat(starting_zeros).as_str()) {
        let digest = md5::compute(format!("{}{}", input, i));
        hash = hex::encode(digest.as_ref());
        i += 1;
    }

    i - 1
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading file");

    println!("{}", part(&input, 5));
    println!("{}", part(&input, 6));
}
