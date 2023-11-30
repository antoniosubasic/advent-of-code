use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(input: &Vec<String>) -> i32 {
    let vovels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden = ["ab", "cd", "pq", "xy"];

    let mut nice = 0;

    for line in input {
        let mut vovel_count = 0;
        let mut double = false;
        let mut is_forbidden = false;
        let mut last_char = '\0';

        for c in line.chars() {
            if vovels.contains(&c) {
                vovel_count += 1;
            }

            if c == last_char {
                double = true;
            }
            
            if forbidden.contains(&format!("{}{}", last_char, c).as_str()) {
                is_forbidden = true;
                break;
            }

            last_char = c;
        }

        if vovel_count >= 3 && double && !is_forbidden {
            nice += 1;
        }
    }

    nice
}

fn part2(input: &Vec<String>) -> i32 {
    let mut nice = 0;

    for line in input {
        let mut pair = false;
        let mut repeat = false;

        for i in 0..line.len() - 2 {
            let pair_str = &line[i..(i + 2)];
            
            if line[(i + 2)..].contains(pair_str) {
                pair = true;
            }

            if line.chars().nth(i) == line.chars().nth(i + 2) {
                repeat = true;
            }
        }

        if pair && repeat {
            nice += 1;
        }
    }

    nice
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
