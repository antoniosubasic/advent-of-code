use std::fs;

fn get_params(input: &Vec<i32>, i: usize) -> (i32, Option<i32>, Option<usize>) {
    let mode1 = input[i] / 100 % 10;
    let mode2 = input[i] / 1000 % 10;

    (
        if mode1 == 0 {
            input[input[i + 1] as usize]
        } else {
            input[i + 1]
        },
        if i + 2 < input.len() {
            if mode2 == 0 {
                let index = input[i + 2] as usize;
                if index < input.len() {
                    Some(input[index])
                } else {
                    None
                }
            } else {
                Some(input[i + 2])
            }
        } else {
            None
        },
        if i + 3 < input.len() {
            Some(input[i + 3] as usize)
        } else {
            None
        },
    )
}

fn run(input: &Vec<i32>, param: i32) -> i32 {
    let mut diag_code = 0;
    let mut input = input.clone();
    let mut i = 0;

    loop {
        let opcode = input[i] % 100;

        if opcode == 99 {
            break;
        } else {
            let (op1, op2, outpos) = get_params(&input, i);

            match opcode {
                1 | 2 => {
                    input[outpos.unwrap()] = if opcode == 1 {
                        op1 + op2.unwrap()
                    } else {
                        op1 * op2.unwrap()
                    };
                    i += 4;
                }
                3 => {
                    let outpos = input[i + 1] as usize;
                    input[outpos] = param;
                    i += 2;
                }
                4 => {
                    diag_code = op1;
                    i += 2;
                }
                5 | 6 => {
                    if if opcode == 5 { op1 != 0 } else { op1 == 0 } {
                        i = op2.unwrap() as usize;
                    } else {
                        i += 3;
                    }
                }
                7 | 8 => {
                    let should_execute = if opcode == 7 {
                        op1 < op2.unwrap()
                    } else {
                        op1 == op2.unwrap()
                    };
                    input[outpos.unwrap()] = if should_execute { 1 } else { 0 };
                    i += 4;
                }
                _ => panic!("invalid opcode: {}", opcode),
            }
        }
    }

    diag_code
}

fn main() {
    let input: Vec<i32> = fs::read_to_string("../input.txt")
        .unwrap()
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}\n{}", run(&input, 1), run(&input, 5));
}
