use std::fs;

fn main() {
    let input: Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line
                .split(&['[', ']', '(', ')', '{', '}'])
                .filter(|part| part.trim() != "");

            let light_diagram = parts
                .next()
                .unwrap()
                .chars()
                .map(|light| light == '#')
                .collect();

            let joltage_requirements = parts
                .next_back()
                .unwrap()
                .split(',')
                .map(|joltage| joltage.parse().unwrap())
                .collect();

            let button_wirings = parts
                .map(|wiring| {
                    wiring
                        .split(',')
                        .map(|wire| wire.parse().unwrap())
                        .collect()
                })
                .collect();

            (light_diagram, button_wirings, joltage_requirements)
        })
        .collect();

    let mut total_min_presses = 0;

    for (target_diagram, button_wirings, _) in &input {
        let mut min_presses = usize::MAX;

        for mask in 1..(1 << button_wirings.len()) {
            let mut diagram = vec![false; target_diagram.len()];
            let mut presses = 0;

            for (i, wiring) in button_wirings.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    presses += 1;
                    for &pos in wiring {
                        diagram[pos] ^= true;
                    }
                }
            }

            if diagram == *target_diagram && presses < min_presses {
                min_presses = presses;
            }
        }

        total_min_presses += min_presses;
    }

    println!("{total_min_presses}");
}
