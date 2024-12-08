use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn add(&mut self, diff: &(i32, i32)) {
        self.x += diff.0;
        self.y += diff.1;
    }

    fn is_valid(&self, map: &Vec<Vec<char>>) -> bool {
        self.y >= 0
            && self.y < map.len() as i32
            && self.x >= 0
            && self.x < map[self.y as usize].len() as i32
    }
}

fn main() {
    let input_string = fs::read_to_string("../input.txt").unwrap();

    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    let mut input: Vec<Vec<char>> = vec![];

    for (y, line) in input_string.lines().enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            antennas.entry(c).or_insert(vec![]).push(Coordinate {
                x: x as i32,
                y: y as i32,
            });
        }

        input.push(line.chars().collect());
    }

    let mut antinodes: (HashSet<Coordinate>, HashSet<Coordinate>) =
        (HashSet::new(), HashSet::new());

    for (_, coords) in &antennas {
        for i in 0..coords.len() - 1 {
            for j in i + 1..coords.len() {
                for (coord, diff) in vec![
                    (
                        coords[i],
                        (coords[i].x - coords[j].x, coords[i].y - coords[j].y),
                    ),
                    (
                        coords[j],
                        (coords[j].x - coords[i].x, coords[j].y - coords[i].y),
                    ),
                ]
                .iter_mut()
                {
                    for i in 0.. {
                        if coord.is_valid(&input) {
                            if i == 1 {
                                antinodes.0.insert(*coord);
                            }
                            antinodes.1.insert(*coord);
                            coord.add(diff);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{}\n{}", antinodes.0.len(), antinodes.1.len());
}
