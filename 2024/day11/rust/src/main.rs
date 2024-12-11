use std::{collections::HashMap, fs};

fn main() {
    let mut input: HashMap<u64, u64> = fs::read_to_string("../input.txt")
        .unwrap()
        .split_whitespace()
        .map(|val| (val.parse().unwrap(), 1))
        .collect();

    for i in 0..75 {
        if i == 25 {
            println!("{}", input.values().sum::<u64>());
        }

        input = input
            .iter()
            .flat_map(|(&stone, count)| {
                {
                    if stone == 0 {
                        vec![1]
                    } else {
                        let length = stone.ilog10() + 1;

                        if length % 2 == 0 {
                            let power = 10_u64.pow(length / 2);
                            vec![stone / power, stone % power]
                        } else {
                            vec![stone * 2024]
                        }
                    }
                }
                .into_iter()
                .map(move |stone| (stone, count))
            })
            .fold(HashMap::new(), |mut acc, (stone, count)| {
                *acc.entry(stone).or_default() += count;
                acc
            });
    }

    println!("{}", input.values().sum::<u64>());
}
