use std::fs;

fn part1(input: &Vec<u32>) -> i32 {
    let mut passwords = 0;

    for i in input[0]..=input[1] {
        let mut adjacent = false;
        let mut increasing = true;

        for j in 0..5 {
            let a = (i / 10u32.pow(5 - j)) % 10;
            let b = (i / 10u32.pow(4 - j)) % 10;

            if a == b {
                adjacent = true;
            }

            if a > b {
                increasing = false;
                break;
            }
        }

        if adjacent && increasing {
            passwords += 1;
        }
    }

    passwords
}

fn part2(input: &Vec<u32>) -> i32 {
    let mut passwords = 0;

    for i in input[0]..=input[1] {
        let mut adjacent = false;
        let mut increasing = true;
        let mut count = 1;

        for j in 0..5 {
            let a = (i / 10u32.pow(5 - j)) % 10;
            let b = (i / 10u32.pow(4 - j)) % 10;

            if a == b {
                count += 1;
            } else {
                if count == 2 {
                    adjacent = true;
                }
                count = 1;
            }

            if a > b {
                increasing = false;
                break;
            }
        }

        if count == 2 || adjacent && increasing {
            passwords += 1;
        }
    }

    passwords
}

fn main() {
    let input: Vec<u32> = fs::read_to_string("../input.txt")
        .expect("failed to read file")
        .split('-')
        .map(|val| val.parse().unwrap())
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
