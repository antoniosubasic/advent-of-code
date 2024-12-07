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

fn get_possible_results(
    operators: &Vec<u64>,
    allowed_operations: &Vec<Operation>,
    expected_result: &u64,
) -> Vec<u64> {
    let mut results = vec![];

    for operation in allowed_operations {
        let result = operation.evaluate(operators[0], operators[1]);

        if result <= *expected_result {
            if operators.len() == 2 {
                results.push(result);
            } else {
                let mut new_operators = vec![result];
                new_operators.extend_from_slice(&operators[2..]);
                results.extend(get_possible_results(
                    &new_operators,
                    allowed_operations,
                    expected_result,
                ));
            }
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

    let allowed_operations = (
        &vec![Operation::Plus, Operation::Multiply],
        &vec![Operation::Plus, Operation::Multiply, Operation::Concatenate],
    );

    let (part1, part2) = input
        .par_iter()
        .map(|(expected_result, operators)| {
            let mut part1 = 0;
            let mut part2 = 0;

            if get_possible_results(operators, allowed_operations.0, expected_result)
                .contains(expected_result)
            {
                part1 += expected_result;
                part2 += expected_result;
            } else if get_possible_results(operators, allowed_operations.1, expected_result)
                .contains(expected_result)
            {
                part2 += expected_result;
            }

            (part1, part2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("{}\n{}", part1, part2);
}
