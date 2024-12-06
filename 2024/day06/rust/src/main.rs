use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn solve(input: &Vec<Vec<char>>) -> Option<HashSet<Coordinate>> {
    let mut current_pos = input
        .iter()
        .enumerate()
        .find_map(|(i, s)| {
            s.iter()
                .enumerate()
                .find_map(|(i, &c)| if c == '^' { Some(i) } else { None })
                .map(|index| Coordinate {
                    x: index as isize,
                    y: i as isize,
                })
        })
        .unwrap();

    let mut positions: HashMap<Coordinate, Vec<Direction>> = HashMap::new();
    let mut current_dir = Direction::North;

    loop {
        if let Some(dirs) = positions.get_mut(&current_pos) {
            if dirs.contains(&current_dir) {
                return None;
            } else {
                dirs.push(current_dir);
            }
        } else {
            positions.insert(current_pos, vec![current_dir]);
        }

        let mut next_pos = current_pos.to_owned();
        match current_dir {
            Direction::North => next_pos.y -= 1,
            Direction::East => next_pos.x += 1,
            Direction::South => next_pos.y += 1,
            Direction::West => next_pos.x -= 1,
        }

        let next_char = if next_pos.y >= 0
            && next_pos.y < input.len() as isize
            && next_pos.x >= 0
            && next_pos.x < input[0].len() as isize
        {
            input
                .get(next_pos.y as usize)
                .unwrap()
                .get(next_pos.x as usize)
        } else {
            None
        };

        if let Some(&next_char) = next_char {
            if next_char == '#' {
                current_dir = match current_dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            } else {
                current_pos = next_pos;
            }
        } else {
            return Some(positions.keys().cloned().collect());
        }
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let positions = solve(&input).unwrap();

    println!("{}", positions.len());

    let mut obstructions = 0;

    for position in positions {
        let x = position.x as usize;
        let y = position.y as usize;

        if input[y][x] == '.' {
            input[y][x] = '#';
            if solve(&input).is_none() {
                obstructions += 1;
            }
            input[y][x] = '.';
        }
    }

    println!("{obstructions}");
}
