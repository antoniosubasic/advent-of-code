use std::fs;

fn get_most_common_bits(numbers: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut bits: Vec<(u32, u32)> = vec![(0, 0); numbers[0].len()];

    for number in numbers {
        for i in 0..number.len() {
            if number[i] {
                bits[i].1 += 1;
            } else {
                bits[i].0 += 1;
            };
        }
    }

    bits.iter().map(|bit| bit.0 <= bit.1).collect()
}

fn u32_from_bits(bits: &Vec<bool>) -> u32 {
    bits.iter()
        .enumerate()
        .map(|(i, &bit)| (bit as u32) << bits.len() - 1 - i)
        .sum()
}

fn main() {
    let input: Vec<Vec<bool>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect();

    let most_common_bits = get_most_common_bits(&input);
    let least_common_bits: Vec<bool> = most_common_bits.iter().map(|bit| !bit).collect();

    println!(
        "{}",
        u32_from_bits(&most_common_bits) * u32_from_bits(&least_common_bits)
    );

    let mut ratings = vec![input.clone(), input.clone()];

    for i in 0.. {
        for j in 0..ratings.len() {
            if ratings[j].len() > 1 {
                let mcb = get_most_common_bits(&ratings[j])[i];

                ratings[j] = ratings[j]
                    .iter()
                    .filter(|number| {
                        if j == 0 {
                            number[i] == mcb
                        } else {
                            number[i] != mcb
                        }
                    })
                    .cloned()
                    .collect();
            }
        }

        if ratings.iter().all(|rating| rating.len() == 1) {
            break;
        }
    }

    println!(
        "{}",
        u32_from_bits(&ratings[0][0]) * u32_from_bits(&ratings[1][0])
    );
}
