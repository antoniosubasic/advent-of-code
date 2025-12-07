use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut splitters: Vec<Splitter> = vec![Splitter {
        coordinate: Coordinate {
            x: input[0].iter().position(|c| c == &'S').unwrap(),
            y: input.iter().position(|line| line.contains(&'^')).unwrap(),
        },
        timelines: 1,
    }];

    for (y, line) in input.iter().enumerate().skip(splitters[0].coordinate.y + 1) {
        for (x, char) in line.iter().enumerate() {
            if char == &'^' {
                // find the nearest splitter directly above the current position
                let threshold_y = splitters
                    .iter()
                    .filter(|splitter| splitter.coordinate.y < y && splitter.coordinate.x == x)
                    .max_by_key(|splitter| splitter.coordinate.y)
                    .map_or(0, |splitter| splitter.coordinate.y);

                // vertical range to be searching for splitters affecting the current splitter
                let y_range = (threshold_y + 1)..y;

                // find splitters 1 to the left or right within the vertical range
                let splitters_offset_above = splitters.iter().filter(|splitter| {
                    y_range.contains(&splitter.coordinate.y)
                        && splitter.coordinate.x.abs_diff(x) == 1
                });

                // sum the timelines of the found splitters
                let timelines = splitters_offset_above
                    .map(|splitter_offset| splitter_offset.timelines)
                    .sum();

                // if the incoming timelines are zero => splitter is not being hit from above, so ignore it
                if timelines != 0 {
                    splitters.push(Splitter {
                        coordinate: Coordinate { x, y },
                        timelines,
                    });
                }
            }
        }
    }

    println!(
        "{}\n{}",
        splitters.len(),
        splitters
            .iter()
            .map(|current| {
                // check if there are any splitters blocking the left path of the current splitter to reach the bottom
                let left_open = !splitters.iter().any(|splitter| {
                    current.coordinate.x > 0
                        && splitter.coordinate.x == current.coordinate.x - 1
                        && splitter.coordinate.y > current.coordinate.y
                });

                // check if there are any splitters blocking the right path of the current splitter to reach the bottom
                let right_open = !splitters.iter().any(|splitter| {
                    splitter.coordinate.x == current.coordinate.x + 1
                        && splitter.coordinate.y > current.coordinate.y
                });

                current.timelines * (left_open as u64 + right_open as u64)
            })
            .sum::<u64>()
    );
}

#[derive(Clone)]
struct Splitter {
    coordinate: Coordinate,
    timelines: u64,
}

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}
