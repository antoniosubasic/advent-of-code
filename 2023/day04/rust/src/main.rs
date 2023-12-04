use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Scratchcard {
    id: i32,
    matching_numbers: usize,
}

impl Scratchcard {
    fn new(id: i32, winning_numbers: Vec<i32>, my_numbers: Vec<i32>) -> Self {
        let matching_numbers = my_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .cloned()
            .collect::<Vec<i32>>()
            .len();

        Scratchcard {
            id,
            matching_numbers,
        }
    }
}

fn part1(input: &mut Vec<Scratchcard>) -> usize {
    let mut sum = 0;

    for card in input.iter() {
        let mut temp_sum = 0;
        for j in 0..card.matching_numbers {
            temp_sum += if j == 0 { 1 } else { temp_sum };
        }

        sum += temp_sum;
    }

    sum
}

fn part2(input: &mut Vec<Scratchcard>) -> i32 {
    let mut card_map = input
        .iter()
        .map(|card| (card.id, 1))
        .collect::<std::collections::HashMap<i32, i32>>();

    for i in 0..input.len() {
        let current_card_amount = card_map.get_key_value(&input[i].id).unwrap().1.clone();
        let cards_to_copy = input[(i + 1)..=(i + input[i].matching_numbers)].to_vec();

        for card in cards_to_copy {
            if let Some(value) = card_map.get_mut(&card.id) {
                *value += current_card_amount;
            }
        }
    }

    card_map.values().sum()
}

fn parse_line(line: &str) -> Scratchcard {
    let mut split = line.split(" | ");

    let mut given = split.next().unwrap().split(": ");

    let id = given
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let given_numbers = given
        .next()
        .unwrap()
        .split(' ')
        .filter(|x| x != &"")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let my_numbers = split
        .next()
        .unwrap()
        .split(' ')
        .filter(|str| str != &"")
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Scratchcard::new(id, given_numbers, my_numbers)
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let mut input: Vec<Scratchcard> = reader
        .lines()
        .map(|line| parse_line(&line.expect("Error reading line")))
        .collect();

    println!("{}", part1(&mut input));
    println!("{}", part2(&mut input));
}
