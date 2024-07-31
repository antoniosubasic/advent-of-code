use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let raw_input: Vec<i16> = reader
        .lines()
        .map(|line| line.expect("could not read line").parse::<i16>().unwrap())
        .collect();

    let mut input = raw_input.clone();
    let mut i = 0;
    let mut steps = 0;

    while i >= 0 && i < input.len() as i16 {
        let jump = input[i as usize];
        input[i as usize] += 1;
        i += jump;
        steps += 1;
    }

    println!("{steps}");

    let mut input = raw_input.clone();
    i = 0;
    steps = 0;

    while i >= 0 && i < input.len() as i16 {
        let jump = input[i as usize];
        input[i as usize] += if jump >= 3 { -1 } else { 1 };
        i += jump;
        steps += 1;
    }

    println!("{steps}");
}
