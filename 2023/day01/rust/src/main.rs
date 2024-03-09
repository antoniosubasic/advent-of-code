use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn part1(input: &Vec<String>) -> i32 {
    let mut sum = 0;

    for str in input {
        let mut first = 0;
        let mut last = 0;

        for i in 0..str.len() {
            let current_first = str.chars().nth(i).unwrap();
            let current_last = str.chars().nth(str.len() - i - 1).unwrap();

            if current_first.is_digit(10) && first == 0 {
                first = current_first.to_digit(10).unwrap() as i32;
            }

            if current_last.is_digit(10) && last == 0 {
                last = current_last.to_digit(10).unwrap() as i32;
            }

            if first != 0 && last != 0 {
                break;
            }
        }

        sum += first * 10 + last;
    }

    sum
}

fn part2(input: &Vec<String>) -> i32 {
    let number_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;

    for str in input {
        let mut numbers = Vec::<i32>::new();

        for mut i in 0..str.len() {
            let current_char = str.chars().nth(i).unwrap();

            if current_char.is_digit(10) {
                numbers.push(current_char.to_digit(10).unwrap() as i32);
            } else {
                let mut buffer = String::new();

                for j in i..str.len() {
                    let current_char = str.chars().nth(j).unwrap();

                    buffer.push(current_char);

                    if number_strings.contains(&buffer.as_str()) {
                        numbers.push(
                            number_strings.iter().position(|&r| r == buffer).unwrap() as i32 + 1,
                        );
                        i += j - 1;
                    }
                }
            }
        }

        sum += numbers[0] * 10 + numbers[numbers.len() - 1];
    }

    sum
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
