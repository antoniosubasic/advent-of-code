use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }
}

fn part1(input: &str) -> i32 {
    let mut location = Location::new();
    let mut visited = vec![Location::new()];

    for c in input.chars() {
        match c {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => (),
        }

        if !visited.contains(&location) {
            visited.push(location);
        }
    };

    visited.len() as i32
}

fn part2(input: &str) -> i32 {
    let mut santa_location = Location::new();
    let mut rb_santa_location = Location::new();
    let mut visited = vec![Location::new()];

    for i in 0..input.len() {
        let c = input.chars().nth(i).unwrap();

        let location = if i % 2 == 0 {
            &mut santa_location
        } else {
            &mut rb_santa_location
        };

        match c {
            '^' => location.y += 1,
            'v' => location.y -= 1,
            '>' => location.x += 1,
            '<' => location.x -= 1,
            _ => (),
        }

        if !visited.contains(location) {
            visited.push(*location);
        }
    }

    visited.len() as i32
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading file");

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
