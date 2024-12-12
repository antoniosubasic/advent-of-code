use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn get_neighbours<T>(&self, grid: &[Vec<T>]) -> Vec<Coordinate> {
        vec![(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .filter_map(|&(dx, dy)| {
                let x = self.x as isize + dx;
                let y = self.y as isize + dy;

                if x >= 0
                    && y >= 0
                    && (y as usize) < grid.len()
                    && (x as usize) < grid[y as usize].len()
                {
                    Some(Coordinate {
                        x: x as usize,
                        y: y as usize,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

macro_rules! hashset {
    ( $( $x: expr),* ) => {
        {
            let mut temp = HashSet::new();
            $(
                temp.insert($x);
            )*
            temp
        }
    };
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut map: HashMap<char, Vec<HashSet<Coordinate>>> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            let coordinate = Coordinate { x, y };
            let neighbours = coordinate.get_neighbours(&input);

            if let Some(regions) = map.get_mut(&char) {
                let matching_regions: Vec<usize> = regions
                    .iter()
                    .enumerate()
                    .filter_map(|(i, region)| {
                        if neighbours.iter().any(|neighbour| {
                            input[neighbour.y][neighbour.x] == char && region.contains(neighbour)
                        }) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();

                if matching_regions.len() > 0 {
                    for i in 1..matching_regions.len() {
                        let region = regions.remove(matching_regions[i]);
                        regions[matching_regions[0]].extend(region);
                    }

                    regions[matching_regions[0]].insert(coordinate);
                } else {
                    regions.push(hashset![coordinate]);
                }
            } else {
                map.insert(char, vec![hashset![coordinate]]);
            }
        }
    }

    let mut price = 0;

    for region in map.values().flatten() {
        let mut perimeter = 0;

        for coord in region {
            if coord.y == 0 || coord.y == input.len() - 1 {
                perimeter += 1;
            }

            if coord.x == 0 || coord.x == input[coord.y].len() - 1 {
                perimeter += 1;
            }

            for neighbour in coord.get_neighbours(&input) {
                if !region.contains(&neighbour) {
                    perimeter += 1;
                }
            }
        }

        price += region.len() * perimeter;
    }

    println!("{price}");
}
