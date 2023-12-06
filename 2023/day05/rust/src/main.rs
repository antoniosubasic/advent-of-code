use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<u32>,
    maps: Vec<Vec<Range>>,
}

impl Input {
    fn new(raw_input: Vec<String>) -> Input {
        let seeds = raw_input[0]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let maps = raw_input[2..]
            .join("\n")
            .split("\n\n")
            .map(|map_item| {
                map_item
                    .split('\n')
                    .skip(1)
                    .map(|range| {
                        Range::new(
                            range
                                .split(' ')
                                .map(|range| range.parse::<u32>().unwrap())
                                .collect::<Vec<u32>>(),
                        )
                    })
                    .collect::<Vec<Range>>()
            })
            .collect::<Vec<Vec<Range>>>();

        Input { seeds, maps }
    }

    fn get_index(&self, map_index: usize, index: u32) -> u32 {
        for range in &self.maps[map_index] {
            if index >= range.source_start && index < (range.source_start + range.length) {
                return index - range.source_start + range.destination_start;
            }
        }

        index
    }
}

#[derive(Debug, Clone)]
struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl Range {
    fn new(range: Vec<u32>) -> Range {
        Range {
            destination_start: range[0],
            source_start: range[1],
            length: range[2],
        }
    }
}

fn part1(input: &Input) -> u32 {
    let mut lowest_location = 0;

    for seed in &input.seeds {
        let mut location_num = *seed;

        for i in 0..input.maps.len() {
            location_num = input.get_index(i, location_num);
        }

        if lowest_location == 0 || location_num < lowest_location {
            lowest_location = location_num;
        }
    }

    lowest_location
}

fn part2(input: &Input) -> u32 {
    let mut lowest_location = 0;

    for i in (0..input.seeds.len()).step_by(2) {
        let seeds = (input.seeds[i]..(input.seeds[i] + input.seeds[i + 1])).collect::<Vec<u32>>();

        for seed in &seeds {
            let mut location_num = *seed;

            for j in 0..input.maps.len() {
                location_num = input.get_index(j, location_num);
            }

            if lowest_location == 0 || location_num < lowest_location {
                lowest_location = location_num;
            }
        }
    }

    lowest_location
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);
    let input = Input::new(
        reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>(),
    );

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
