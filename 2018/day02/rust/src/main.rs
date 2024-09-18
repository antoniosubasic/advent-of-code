use std::{
    collections::HashMap,
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

    let mut total = (0, 0);
    let mut common: Option<String> = None;

    for i in 0..input.len() {
        let mut chars: HashMap<char, i32> = HashMap::new();

        for char in input[i].chars() {
            *chars.entry(char).or_insert(0) += 1;
        }

        if chars.values().any(|&v| v == 2) {
            total.0 += 1;
        }

        if chars.values().any(|&v| v == 3) {
            total.1 += 1;
        }

        if common == None {
            for j in i + 1..input.len() {
                let diff: Vec<bool> = input[i]
                    .chars()
                    .zip(input[j].chars())
                    .map(|(a, b)| a == b)
                    .collect();

                if diff.iter().filter(|&&d| !d).count() == 1 {
                    common = Some(
                        input[i]
                            .chars()
                            .zip(diff.iter())
                            .filter_map(|(c, &d)| if d { Some(c) } else { None })
                            .collect(),
                    );
                }
            }
        }
    }

    println!("{}\n{}", total.0 * total.1, common.unwrap());
}
