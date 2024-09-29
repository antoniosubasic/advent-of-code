use std::fs;

fn main() {
    let input: Vec<u16> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut increased = (0, 0);

    for i in 0..input.len() - 1 {
        if input[i + 1] > input[i] {
            increased.0 += 1;
        }

        if i < input.len() - 3 && input[i + 3] > input[i] {
            increased.1 += 1;
        }
    }

    println!("{}\n{}", increased.0, increased.1);
}
