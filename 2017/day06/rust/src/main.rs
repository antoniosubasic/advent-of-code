use std::fs;

fn cycle(input: &mut Vec<i32>) {
    let max = input.iter().max().unwrap();
    let max_index = input.iter().position(|&x| x == *max).unwrap();

    let blocks = input[max_index] as usize;
    input[max_index] = 0;

    let input_len = input.len();

    for i in 1..=blocks {
        input[(max_index + i) % input_len] += 1;
    }
}

fn find_reoccurance(input: &mut Vec<i32>) -> usize {
    let mut seen: Vec<Vec<i32>> = Vec::new();

    while !seen.contains(&input) {
        seen.push(input.clone());
        cycle(input);
    }

    seen.len()
}

fn main() {
    let mut input: Vec<i32> = fs::read_to_string("../input.txt")
        .unwrap()
        .split('\t')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    println!("{}", find_reoccurance(&mut input));
    println!("{}", find_reoccurance(&mut input));
}
