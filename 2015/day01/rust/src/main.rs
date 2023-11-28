use std::fs;

fn part1(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    };

    floor
}

fn part2(input: &str) -> Result<i32, &'static str> {
    let mut floor = 0;

    for i in 0..input.len() {
        match input.chars().nth(i) {
            Some('(') => floor += 1,
            Some(')') => floor -= 1,
            _ => (),
        };

        if floor < 0 {
            return Ok(i as i32 + 1);
        }
    };
    
    Err("Santa never enters the basement")
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading file");
    
    println!("{}", part1(&input));
    println!("{}", part2(&input).expect("Error calculating part2"));
}
