use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split('\t')
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut checksum_part1 = 0;
    let mut checksum_part2 = 0;

    for line in input {
        checksum_part1 += line.iter().max().unwrap() - line.iter().min().unwrap();

        for i in 0..line.len() {
            for j in (i + 1)..line.len() {
                if line[i] % line[j] == 0 {
                    checksum_part2 += line[i] / line[j];
                } else if line[j] % line[i] == 0 {
                    checksum_part2 += line[j] / line[i];
                }
            }
        }
    }

    println!("{checksum_part1}\n{checksum_part2}");
}
