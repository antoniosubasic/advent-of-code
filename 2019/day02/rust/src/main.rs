use std::fs;

fn run(input: &Vec<u32>) -> u32 {
    let mut input = input.clone();
    let mut i = 0;

    loop {
        match input[i] {
            1 | 2 => {
                let (op1, op2, outpos) = (
                    input[input[i + 1] as usize],
                    input[input[i + 2] as usize],
                    input[i + 3] as usize,
                );

                input[outpos] = if input[i] == 1 { op1 + op2 } else { op1 * op2 };

                i += 4;
            }
            99 => break,
            _ => panic!("invalid opcode: {}", input[i]),
        }
    }

    input[0]
}

fn main() {
    let mut input: Vec<u32> = fs::read_to_string("../input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    input[1] = 12;
    input[2] = 2;

    println!("{}", run(&input));

    for noun in 0..100 {
        for verb in 0..100 {
            (input[1], input[2]) = (noun, verb);

            if run(&input) == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}
