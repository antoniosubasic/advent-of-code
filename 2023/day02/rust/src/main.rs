use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    cube_subsets: Vec<Vec<(i32, Color)>>,
}

impl Game {
    fn get_max_number_of_cubes(self, color: Color) -> i32 {
        let mut max = 0;

        for subset in self.cube_subsets {
            let mut subset_max = 0;

            for cube in subset {
                if cube.1 == color {
                    subset_max += cube.0;
                }
            }

            if subset_max > max {
                max = subset_max;
            }
        }

        max
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    Red = 12,
    Green = 13,
    Blue = 14,
}

fn part1(input: &Vec<Game>) -> i32 {
    let mut sum = 0;

    for game in input {
        let mut valid_game = true;

        for subset in &game.cube_subsets {
            for cube in subset {
                if cube.0 > cube.1 as i32 {
                    valid_game = false;
                    break;
                }
            }
        }

        if valid_game {
            sum += game.id;
        }
    }

    sum
}

fn part2(input: &Vec<Game>) -> i32 {
    let mut sum = 0;

    for game in input {
        sum += game.clone().get_max_number_of_cubes(Color::Red) * game.clone().get_max_number_of_cubes(Color::Green) * game.clone().get_max_number_of_cubes(Color::Blue);
    }

    sum
}

fn parse_line(line: &str) -> Game {
    let mut splitted = line.split(':');

    let id = splitted.next().unwrap().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
    let mut subsets = Vec::<Vec<(i32, Color)>>::new();

    for subset in splitted.next().unwrap().split(';') {
        let mut cubes = Vec::<(i32, Color)>::new();

        for cube_item in subset.split(',') {
            let cube_tuple: Vec<&str> = cube_item.trim().split(' ').collect();

            cubes.push((
                cube_tuple[0].parse::<i32>().unwrap(),
                match cube_tuple[1] {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => panic!("Invalid color"),
                },
            ));
        }

        subsets.push(cubes);
    }

    Game {
        id,
        cube_subsets: subsets,
    }
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let input: Vec<Game> = reader
        .lines()
        .map(|line| parse_line(&line.expect("Error reading line")))
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
