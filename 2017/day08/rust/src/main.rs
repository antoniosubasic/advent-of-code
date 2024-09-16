use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

enum CompareType {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

struct Condition {
    increment: i32,
    variable: String,
    comparetype: CompareType,
    value: i32,
}

struct Instruction {
    register_name: String,
    condition: Condition,
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line: Vec<Vec<&str>> = line
                .split(" if ")
                .map(|part| part.split(' ').collect::<Vec<&str>>())
                .collect();

            let diff_value: i32 = line[0][2].parse().unwrap();

            Instruction {
                register_name: line[0][0].to_string(),
                condition: Condition {
                    increment: if line[0][1] == "inc" {
                        diff_value
                    } else {
                        diff_value * -1
                    },
                    variable: line[1][0].to_string(),
                    comparetype: match line[1][1] {
                        ">" => CompareType::GreaterThan,
                        ">=" => CompareType::GreaterThanOrEqual,
                        "<" => CompareType::LessThan,
                        "<=" => CompareType::LessThanOrEqual,
                        "==" => CompareType::Equal,
                        "!=" => CompareType::NotEqual,
                        _ => panic!("invalid comparetype"),
                    },
                    value: line[1][2].parse().unwrap(),
                },
            }
        })
        .collect();

    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest_value_during_proccess = 0;

    for instruction in &input {
        let mut register_value = *registers.get(&instruction.register_name).unwrap_or(&0);
        let condition_register_value =
            *registers.get(&instruction.condition.variable).unwrap_or(&0);

        if match instruction.condition.comparetype {
            CompareType::GreaterThan => condition_register_value > instruction.condition.value,
            CompareType::GreaterThanOrEqual => {
                condition_register_value >= instruction.condition.value
            }
            CompareType::LessThan => condition_register_value < instruction.condition.value,
            CompareType::LessThanOrEqual => condition_register_value <= instruction.condition.value,
            CompareType::Equal => condition_register_value == instruction.condition.value,
            CompareType::NotEqual => condition_register_value != instruction.condition.value,
        } {
            register_value += instruction.condition.increment;
        }

        if register_value > highest_value_during_proccess {
            highest_value_during_proccess = register_value;
        }

        registers.insert(instruction.register_name.clone(), register_value);
    }

    let mut sorted_registers: Vec<(&String, &i32)> = registers.iter().collect();
    sorted_registers.sort_by(|a, b| b.1.cmp(a.1));

    println!(
        "{}\n{}",
        sorted_registers.first().unwrap().1,
        highest_value_during_proccess
    );
}
