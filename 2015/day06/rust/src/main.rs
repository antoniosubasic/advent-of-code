use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Operation {
    from: (i32, i32),
    to: (i32, i32),
    action: Action,
}

fn part1(input: &Vec<Operation>) -> usize {
    let grid_size = 1000;
    let mut grid = vec![false; grid_size * grid_size];

    for operation in input {
        for i in operation.from.1..=operation.to.1 {
            for j in operation.from.0..=operation.to.0 {
                let index = (i as usize) * grid_size + (j as usize);
                grid[index] = match operation.action {
                    Action::On => true,
                    Action::Off => false,
                    Action::Toggle => !grid[index],
                }
            }
        }
    }

    grid.iter().filter(|&&x| x == true).count()
}

fn part2(input: &Vec<Operation>) -> i32 {
    let grid_size = 1000;
    let mut grid: Vec<i32> = vec![0; grid_size * grid_size];

    for operation in input {
        for i in operation.from.1..=operation.to.1 {
            for j in operation.from.0..=operation.to.0 {
                let index = (i as usize) * grid_size + (j as usize);
                grid[index] += match operation.action {
                    Action::On => 1,
                    Action::Off => if grid[index] > 0 { -1 } else { 0 },
                    Action::Toggle => 2,
                }
            }
        }
    }

    grid.iter().sum()
}

fn parse_line(line: &str) -> Operation {
    let mut split = match line.strip_prefix("turn ") {
        Some(x) => x.split(' '),
        None => line.split(' '),
    };

    let action = match split.next().unwrap() {
        "on" => Action::On,
        "off" => Action::Off,
        "toggle" => Action::Toggle,
        _ => panic!("Invalid action"),
    };

    let from = split
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let to = split
        .nth(1)
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Operation {
        from: (from[0], from[1]),
        to: (to[0], to[1]),
        action,
    }
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let input: Vec<Operation> = reader
        .lines()
        .map(|line| parse_line(&line.expect("Error reading line")))
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
