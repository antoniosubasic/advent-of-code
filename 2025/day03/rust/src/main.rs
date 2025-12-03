use std::fs;

fn largest_joltage(number: &Vec<u8>, digits: usize) -> u64 {
    let mut stack: Vec<u8> = Vec::with_capacity(number.len());
    let mut drops = number.len() - digits;

    for digit in number {
        while stack.last().map_or(false, |last| last < digit && drops > 0) {
            stack.pop();
            drops -= 1;
        }

        stack.push(*digit);
    }

    stack[..digits]
        .iter()
        .enumerate()
        .fold(0_u64, |acc, (i, digit)| {
            acc + *digit as u64 * 10_u64.pow((digits - i - 1) as u32)
        })
}

fn main() {
    let input: Vec<Vec<u8>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let total_joltages = input.iter().fold((0, 0), |acc, number| {
        (
            acc.0 + largest_joltage(number, 2),
            acc.1 + largest_joltage(number, 12),
        )
    });

    println!("{}\n{}", total_joltages.0, total_joltages.1);
}
