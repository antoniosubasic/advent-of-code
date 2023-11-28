use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Present {
    length: i32,
    width: i32,
    height: i32,
}

fn part1(input: &Vec<Present>) -> i32 {
    let mut ribbon = 0;

    for present in input {
        ribbon += 2 * present.length * present.width
            + 2 * present.width * present.height
            + 2 * present.height * present.length;

        let mut sides = vec![present.length, present.width, present.height];
        sides.sort();

        ribbon += sides[0] * sides[1];
    }

    ribbon
}

fn part2(input: &Vec<Present>) -> i32 {
    let mut ribbon = 0;

    for present in input {
        let mut sides = vec![present.length, present.width, present.height];
        sides.sort();

        ribbon += 2 * sides[0] + 2 * sides[1] + present.length * present.width * present.height;
    }

    ribbon
}

fn parse_line(line: &str) -> Present {
    let mut split = line.split('x');
    let length = split.next().unwrap().parse::<i32>().unwrap();
    let width = split.next().unwrap().parse::<i32>().unwrap();
    let height = split.next().unwrap().parse::<i32>().unwrap();

    Present {
        length,
        width,
        height,
    }
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let input: Vec<Present> = reader
        .lines()
        .map(|line| parse_line(&line.expect("Error reading line")))
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
