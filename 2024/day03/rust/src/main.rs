use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let (mut part1, mut part2) = (0, 0);
    let mut should_execute = true;

    for capture in regex.captures_iter(&input) {
        if let (Some(op1), Some(op2)) = (capture.get(1), capture.get(2)) {
            let result =
                op1.as_str().parse::<i32>().unwrap() * op2.as_str().parse::<i32>().unwrap();

            part1 += result;
            if should_execute {
                part2 += result;
            }
        } else {
            should_execute = capture.get(0).unwrap().as_str() != "don't()";
        }
    }

    println!("{part1}\n{part2}");
}
