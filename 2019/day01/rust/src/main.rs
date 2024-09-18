use std::fs;

fn main() {
    let input: Vec<u32> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut total_fuel = (0, 0);

    for val in input {
        let mut fuel = val / 3 - 2;

        total_fuel.0 += fuel;

        loop {
            total_fuel.1 += fuel;

            match fuel.checked_div(3).and_then(|res| res.checked_sub(2)) {
                Some(value) => fuel = value,
                None => break,
            }
        }
    }

    println!("{}\n{}", total_fuel.0, total_fuel.1);
}
