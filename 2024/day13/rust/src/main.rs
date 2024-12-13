use std::fs;

struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn from_tuple(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<(Button, Button, (usize, usize))> = input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();

            let parse_line = |line: &str| {
                let coords = line.split_once(": ").unwrap().1;
                let (x, y) = coords.split_once(", ").unwrap();
                (
                    x[2..].parse::<usize>().unwrap(),
                    y[2..].parse::<usize>().unwrap(),
                )
            };

            (
                Button::from_tuple(parse_line(lines.next().unwrap())),
                Button::from_tuple(parse_line(lines.next().unwrap())),
                parse_line(lines.next().unwrap()),
            )
        })
        .collect();

    let mut min_prize = 0;

    for machine in input {
        let mut min_presses: Option<(usize, usize)> = None;

        let mut presses1 = 0;
        while presses1 * machine.0.x < machine.2 .0 && presses1 * machine.0.y < machine.2 .1 {
            let diff = machine.2 .0 - presses1 * machine.0.x;

            if diff % machine.1.x == 0 {
                let presses2 = diff / machine.1.x;

                if presses1 * machine.0.x + presses2 * machine.1.x == machine.2 .0
                    && presses1 * machine.0.y + presses2 * machine.1.y == machine.2 .1
                {
                    if let Some(min_presses) = &mut min_presses {
                        if presses1 * 3 + presses2 < min_presses.0 * 3 + min_presses.1 {
                            *min_presses = (presses1, presses2);
                        }
                    } else {
                        min_presses = Some((presses1, presses2));
                    }
                }
            }

            presses1 += 1;
        }

        if let Some(min_presses) = min_presses {
            min_prize += min_presses.0 * 3 + min_presses.1;
        }
    }

    println!("{min_prize}");
}
