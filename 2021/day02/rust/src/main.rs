use std::fs;

struct Position {
    depth: i32,
    horizontal: i32,
    aim: Option<i32>,
}

impl Position {
    fn execute_instruction(&mut self, instruction: &(&str, i32)) {
        match instruction.0 {
            "forward" => {
                self.horizontal += instruction.1;

                if let Some(aim) = self.aim {
                    self.depth += aim * instruction.1;
                }
            }
            "down" | "up" => {
                let target = if let Some(ref mut aim) = self.aim {
                    aim
                } else {
                    &mut self.depth
                };

                *target += if instruction.0 == "down" {
                    instruction.1
                } else {
                    -instruction.1
                };
            }
            _ => {}
        }
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(d, v)| (d, v.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let mut parts = vec![
        Position {
            depth: 0,
            horizontal: 0,
            aim: None,
        },
        Position {
            depth: 0,
            horizontal: 0,
            aim: Some(0),
        },
    ];

    for instruction in &input {
        for part in &mut parts {
            part.execute_instruction(instruction);
        }
    }

    println!(
        "{}\n{}",
        parts[0].depth * parts[0].horizontal,
        parts[1].depth * parts[1].horizontal,
    );
}
