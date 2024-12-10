use std::{collections::HashSet, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn get_neighbours(&self) -> Vec<Coordinate> {
        vec![(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .filter_map(|offset| {
                if offset.0 < 0 && self.x == 0 || offset.1 < 0 && self.y == 0 {
                    None
                } else {
                    Some(Self {
                        x: self.x + offset.0 as usize,
                        y: self.y + offset.1 as usize,
                    })
                }
            })
            .collect()
    }
}

fn get_paths(map: &Vec<Vec<u8>>, given_path: Vec<Coordinate>) -> HashSet<Vec<Coordinate>> {
    let mut path = given_path.clone();
    let mut paths = HashSet::new();

    loop {
        let coordinate = *path.last().unwrap();
        let mut found = false;

        for neighbour in coordinate.get_neighbours() {
            if let Some(line) = map.get(neighbour.y) {
                if let Some(digit) = line.get(neighbour.x) {
                    if *digit == map[coordinate.y][coordinate.x] + 1 {
                        found = true;

                        if *path.last().unwrap() == coordinate {
                            path.push(neighbour);
                        } else {
                            let mut path_clone = path.clone();
                            path_clone.push(neighbour);
                            paths.extend(get_paths(map, path_clone));
                        }
                    }
                }
            }
        }

        if !found {
            if map[coordinate.y][coordinate.x] == 9 {
                paths.insert(path);
            }

            return paths;
        }
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    let mut starts: Vec<Coordinate> = vec![];
    let input: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let digit = c.to_digit(10).unwrap() as u8;
                    if digit == 0 {
                        starts.push(Coordinate { x, y });
                    }
                    digit
                })
                .collect()
        })
        .collect();

    let (part1, part2) = starts
        .iter()
        .map(|&start| {
            let paths = get_paths(&input, vec![start]);
            (
                paths
                    .iter()
                    .map(|path| path.last().unwrap())
                    .collect::<HashSet<&Coordinate>>()
                    .len(),
                paths.len(),
            )
        })
        .reduce(|acc, item| (acc.0 + item.0, acc.1 + item.1))
        .unwrap();

    println!("{}\n{}", part1, part2);
}
