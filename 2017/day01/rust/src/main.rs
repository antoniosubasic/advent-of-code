use std::fs;

fn main() {
    let input: Vec<char> = fs::read_to_string("../input.txt")
        .unwrap()
        .chars()
        .collect();

    let mut captcha_part1 = 0;
    let mut captcha_part2 = 0;

    for i in 0..input.len() {
        if input.get(i) == input.get((i + 1) % input.len()) {
            captcha_part1 += input[i].to_digit(10).unwrap();
        }

        if input.get(i) == input.get((i + input.len() / 2) % input.len()) {
            captcha_part2 += input[i].to_digit(10).unwrap();
        }
    }

    println!("{captcha_part1}\n{captcha_part2}");
}
