use std::{collections::HashMap, fs};

struct CoordinateInfo {
    crossed: (bool, bool),
    steps: (usize, usize),
}

fn main() {
    let input: Vec<Vec<(char, i32)>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| {
                    let (direction, val) = s.split_at(1);
                    (direction.chars().next().unwrap(), val.parse().unwrap())
                })
                .collect::<Vec<(char, i32)>>()
        })
        .collect();

    let mut map: HashMap<(i32, i32), CoordinateInfo> = HashMap::new();
    let mut pos = ((0, 0), (0, 0));
    let mut steps = (0, 0);

    for i in 0..input[0].len() {
        for j in 0..input.len() {
            for _ in 0..input[j][i].1 {
                let pos = if j == 0 { &mut pos.0 } else { &mut pos.1 };

                match map.get_mut(&pos) {
                    Some(cord_info) => {
                        if j == 0 {
                            cord_info.crossed.0 = true;
                            cord_info.steps.0 = steps.0;
                        } else {
                            cord_info.crossed.1 = true;
                            cord_info.steps.1 = steps.1;
                        }
                    }
                    None => {
                        map.insert(
                            *pos,
                            if j == 0 {
                                CoordinateInfo {
                                    crossed: (true, false),
                                    steps: (steps.0, 0),
                                }
                            } else {
                                CoordinateInfo {
                                    crossed: (false, true),
                                    steps: (0, steps.1),
                                }
                            },
                        );
                    }
                }

                if j == 0 {
                    steps.0 += 1;
                } else {
                    steps.1 += 1;
                }

                match input[j][i].0 {
                    'U' => pos.1 += 1,
                    'R' => pos.0 += 1,
                    'D' => pos.1 -= 1,
                    'L' => pos.0 -= 1,
                    _ => panic!("invalid direction: {}", input[j][i].0),
                }
            }
        }
    }

    let mut crosses: Vec<(&(i32, i32), &CoordinateInfo)> = map
        .iter()
        .filter(|coordinate| coordinate.1.crossed.0 && coordinate.1.crossed.1)
        .filter(|coordinate| coordinate.0 .0 != 0 || coordinate.0 .1 != 0)
        .collect();

    crosses.sort_by(|a, b| (a.0 .0.abs() + a.0 .1.abs()).cmp(&(b.0 .0.abs() + b.0 .1.abs())));
    let closest_cross = crosses.first().unwrap();
    println!("{}", closest_cross.0 .0.abs() + closest_cross.0 .1.abs());

    crosses.sort_by(|a, b| (a.1.steps.0 + a.1.steps.1).cmp(&(b.1.steps.0 + b.1.steps.1)));
    let closest_cross = crosses.first().unwrap();
    println!("{}", closest_cross.1.steps.0 + closest_cross.1.steps.1);
}
