use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    let mut valid_passwords_part1 = 0;
    let mut valid_passwords_part2 = 0;

    for line in input {
        let passwords_vec: Vec<String> = line.split(' ').map(|pw| pw.chars().collect()).collect();

        if passwords_vec.len()
            == passwords_vec
                .clone()
                .into_iter()
                .collect::<HashSet<String>>()
                .len()
        {
            valid_passwords_part1 += 1;
        }

        let passwords_vec: Vec<String> = passwords_vec
            .iter()
            .map(|string| {
                let mut chars: Vec<char> = string.chars().collect();
                chars.sort();
                chars.into_iter().collect()
            })
            .collect();

        if passwords_vec.len()
            == passwords_vec
                .clone()
                .into_iter()
                .collect::<HashSet<String>>()
                .len()
        {
            valid_passwords_part2 += 1;
        }
    }

    println!("{valid_passwords_part1}\n{valid_passwords_part2}");
}
