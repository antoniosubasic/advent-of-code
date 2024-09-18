use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

struct Square {
    id: u16,
    start: (i32, i32),
    dimensions: (i32, i32),
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<Square> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(' ').collect();

            let id = parts[0].trim_start_matches('#').parse::<u16>().unwrap();
            let start: Vec<i32> = parts[2]
                .trim_end_matches(':')
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            let dimensions: Vec<i32> = parts[3]
                .split('x')
                .map(|v| v.parse::<i32>().unwrap())
                .collect();

            Square {
                id,
                start: (start[0], start[1]),
                dimensions: (dimensions[0], dimensions[1]),
            }
        })
        .collect();

    let mut coordinates: HashMap<(i32, i32), i32> = HashMap::new();

    for square in &input {
        for x in (square.start.0)..(square.start.0 + square.dimensions.0) {
            for y in (square.start.1)..(square.start.1 + square.dimensions.1) {
                *coordinates.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    println!(
        "{}",
        coordinates.iter().filter(|coord| *coord.1 >= 2).count()
    );

    for square in &input {
        let mut overlaps = false;

        for x in (square.start.0)..(square.start.0 + square.dimensions.0) {
            for y in (square.start.1)..(square.start.1 + square.dimensions.1) {
                if *coordinates.get(&(x, y)).unwrap() >= 2 {
                    overlaps = true;
                }
            }
        }

        if !overlaps {
            println!("{}", square.id);
            break;
        }
    }
}
