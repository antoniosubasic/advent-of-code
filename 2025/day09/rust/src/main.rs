use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs;

fn main() {
    let input: Vec<Coordinate> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Coordinate {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let mut max_area = (0, 0);

    for (i, rec1) in input.iter().enumerate() {
        for rec2 in input.iter().skip(i + 1) {
            let area = (rec1.x.abs_diff(rec2.x) + 1) * (rec1.y.abs_diff(rec2.y) + 1);

            if area > max_area.0 {
                max_area.0 = area;
            }

            if area > max_area.1 {
                let inside_x_range = (rec1.x.min(rec2.x) + 1)..rec1.x.max(rec2.x);
                let inside_y_range = (rec1.y.min(rec2.y) + 1)..rec1.y.max(rec2.y);

                let intersects = (0..(input.len() - 1)).into_par_iter().any(|i| {
                    let (start, end) = (input[i], input[i + 1]);

                    let line: Vec<_> = if start.x == end.x {
                        if !inside_x_range.contains(&start.x) {
                            return false;
                        }

                        (start.y.min(end.y)..=start.y.max(end.y))
                            .map(|y| Coordinate { x: start.x, y })
                            .collect()
                    } else if start.y == end.y {
                        if !inside_y_range.contains(&start.y) {
                            return false;
                        }

                        (start.x.min(end.x)..=start.x.max(end.x))
                            .map(|x| Coordinate { x, y: start.y })
                            .collect()
                    } else {
                        unreachable!()
                    };

                    line.par_iter().any(|coord| {
                        inside_x_range.contains(&coord.x) && inside_y_range.contains(&coord.y)
                    })
                });

                if !intersects {
                    max_area.1 = area;
                }
            }
        }
    }

    println!("{}\n{}", max_area.0, max_area.1);
}

#[derive(PartialEq, Clone, Copy)]
struct Coordinate {
    x: u64,
    y: u64,
}
