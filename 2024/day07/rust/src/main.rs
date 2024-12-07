use rayon::prelude::*;
use std::fs;

enum Operation {
    Plus,
    Multiply,
    Concatenate,
}

impl Operation {
    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Plus => a + b,
            Self::Multiply => a * b,
            Self::Concatenate => a * 10_u64.pow((b as f64).log10().floor() as u32 + 1) + b,
        }
    }
}

fn get_possible_results(operators: &Vec<u64>, operations: &Vec<Operation>) -> Vec<u64> {
    let mut results = vec![];

    for operation in operations {
        let result = operation.evaluate(operators[0], operators[1]);

        if operators.len() == 2 {
            results.push(result);
        } else {
            let mut new_operators = vec![result];
            new_operators.extend_from_slice(&operators[2..]);
            results.extend(get_possible_results(&new_operators, operations));
        }
    }

    results
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let (result, operands) = (line.split_once(": ")).unwrap();
            (
                result.parse().unwrap(),
                operands
                    .split_whitespace()
                    .map(|operand| operand.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let (part1, part2) = input
        .par_iter()
        .map(|equation| {
            let mut part1 = 0;
            let mut part2 = 0;

            if get_possible_results(&equation.1, &vec![Operation::Plus, Operation::Multiply])
                .contains(&equation.0)
            {
                part1 += equation.0;
                part2 += equation.0;
            } else if get_possible_results(
                &equation.1,
                &vec![Operation::Plus, Operation::Multiply, Operation::Concatenate],
            )
            .contains(&equation.0)
            {
                part2 += equation.0;
            }

            (part1, part2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}\n{}", part1, part2);
}
