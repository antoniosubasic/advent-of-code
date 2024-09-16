use std::{collections::HashSet, fs};

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<&str> = input.split(", ").collect();

    let mut location = Coordinate { x: 0, y: 0 };
    let mut direction = Direction::North;

    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut first_visited_twice: Option<Coordinate> = None;

    for movement in input {
        let mut movement = movement.chars();

        direction = match movement.next().unwrap() {
            'R' => match direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            'L' => match direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            _ => direction,
        };

        let steps = movement.collect::<String>().parse::<i32>().unwrap();

        if first_visited_twice == None {
            for _ in 0..steps {
                if first_visited_twice == None {
                    if visited.contains(&location) {
                        first_visited_twice = Some(location.clone());
                    } else {
                        visited.insert(location.clone());
                    }
                }

                match direction {
                    Direction::North => location.y += 1,
                    Direction::East => location.x += 1,
                    Direction::South => location.y -= 1,
                    Direction::West => location.x -= 1,
                }
            }
        } else {
            match direction {
                Direction::North => location.y += steps,
                Direction::East => location.x += steps,
                Direction::South => location.y -= steps,
                Direction::West => location.x -= steps,
            }
        }
    }

    println!(
        "{}\n{}",
        location.manhattan_distance(),
        first_visited_twice.unwrap().manhattan_distance()
    );
}
