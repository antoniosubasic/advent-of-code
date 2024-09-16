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

    let mut part1 = String::new();
    let mut part2 = String::new();

    for i in 0..input[0].len() {
        let mut frequency: HashMap<char, i32> = HashMap::new();

        for j in 0..input.len() {
            let char = input[j].chars().nth(i).unwrap();

            match frequency.get_mut(&char) {
                Some(entry) => {
                    *entry += 1;
                }
                None => {
                    frequency.insert(char, 1);
                }
            }
        }

        let mut frequency_vec: Vec<(&char, &i32)> = frequency.iter().collect();
        frequency_vec.sort_by(|a, b| b.1.cmp(a.1));

        part1.push(*frequency_vec.first().unwrap().0);
        part2.push(*frequency_vec.last().unwrap().0);
    }

    println!("{part1}\n{part2}");
}
