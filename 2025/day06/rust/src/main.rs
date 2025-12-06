use std::fs;

fn solve(matrix: &Vec<Vec<u64>>, operators: &Vec<Operator>) -> u64 {
    let mut results: Vec<u64> = operators
        .iter()
        .map(|operator| operator.initial_value())
        .collect();

    for numbers in matrix {
        for (i, &number) in numbers.iter().enumerate() {
            if number != 0 {
                results[i] = operators[i].apply(results[i], number);
            }
        }
    }

    results.iter().sum::<u64>()
}

fn main() {
    let input: Vec<String> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let operators: Vec<(Operator, usize)> = input[input.len() - 1]
        .chars()
        .enumerate()
        .filter_map(|(i, char)| Operator::parse(char).map(|operator| (operator, i)))
        .collect();

    let part1_matrix: Vec<Vec<u64>> = input
        .iter()
        .take(input.len() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let mut part2_matrix: Vec<Vec<u64>> = vec![vec![0; part1_matrix[0].len()]; part1_matrix.len()];

    // constructing part2_matrix
    {
        let input: Vec<Vec<String>> = input
            .iter()
            .take(input.len() - 1)
            .map(|line| {
                let mut parsed_line: Vec<String> = Vec::with_capacity(operators.len());

                for i in 1..=operators.len() {
                    let start = operators[i - 1].1;
                    let end = (operators.get(i).map(|(_, index)| index - 1)).unwrap_or(line.len());
                    parsed_line.push(line[start..end].to_string());
                }

                parsed_line
            })
            .collect();

        for line in &input {
            for x in 0..input[0].len() {
                for (i, digit) in line[x].chars().enumerate() {
                    if digit != ' ' {
                        let row = part2_matrix.len() - i - 1;
                        let col = part2_matrix[0].len() - x - 1;

                        part2_matrix[row][col] *= 10;
                        part2_matrix[row][col] += digit.to_digit(10).unwrap() as u64;
                    }
                }
            }
        }
    }

    let operators: Vec<Operator> = operators.into_iter().map(|(op, _)| op).collect();

    println!(
        "{}\n{}",
        solve(&part1_matrix, &operators),
        solve(&part2_matrix, &operators.into_iter().rev().collect())
    );
}

enum Operator {
    Multiply,
    Add,
}

impl Operator {
    fn parse(char: char) -> Option<Self> {
        match char {
            '*' => Some(Operator::Multiply),
            '+' => Some(Operator::Add),
            _ => None,
        }
    }

    fn apply(&self, number1: u64, number2: u64) -> u64 {
        match self {
            Operator::Multiply => number1 * number2,
            Operator::Add => number1 + number2,
        }
    }

    fn initial_value(&self) -> u64 {
        match self {
            Operator::Multiply => 1,
            Operator::Add => 0,
        }
    }
}
