use std::fs;

fn main() {
    let input: Vec<u32> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut parts: (Option<u32>, Option<u32>) = (None, None);

    'outer: for i in 0..input.len() {
        for j in i + 1..input.len() {
            if parts.0 == None && input[i] + input[j] == 2020 {
                parts.0 = Some(input[i] * input[j]);
            }

            if parts.1 == None {
                for k in j + 1..input.len() {
                    if input[i] + input[j] + input[k] == 2020 {
                        parts.1 = Some(input[i] * input[j] * input[k]);
                    }
                }
            }

            if let (Some(_), Some(_)) = parts {
                break 'outer;
            }
        }
    }

    println!(
        "{}\n{}",
        parts.0.expect("no solution found for part 1"),
        parts.1.expect("no solution found for part 2")
    );
}
