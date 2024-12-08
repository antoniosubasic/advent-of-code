use rayon::prelude::*;
use std::fs;

#[derive(Debug)]
struct Range {
    dest: u32,
    src: u32,
    len: u32,
}

impl Range {
    fn apply_to(&self, val: u32) -> Option<u32> {
        if val >= self.src && val < self.src + self.len {
            Some(self.dest + (val - self.src))
        } else {
            None
        }
    }
}

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds: Vec<u32> = seeds
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().unwrap())
        .collect();

    let maps: Vec<Vec<Range>> = maps
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let mut split = line.split_whitespace();
                    Range {
                        dest: split.next().unwrap().parse().unwrap(),
                        src: split.next().unwrap().parse().unwrap(),
                        len: split.next().unwrap().parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    let transform_seed = |seed: &u32| {
        let mut seed = *seed;

        for map in &maps {
            for range in map {
                if let Some(val) = range.apply_to(seed) {
                    seed = val;
                    break;
                }
            }
        }

        seed
    };

    println!(
        "{}\n{}",
        seeds
            .par_iter()
            .map(|seed| transform_seed(seed))
            .min()
            .unwrap(),
        seeds
            .par_chunks(2)
            .map(|seeds| (seeds[0]..seeds[0] + seeds[1])
                .into_par_iter()
                .map(|seed| transform_seed(&seed))
                .min()
                .unwrap())
            .min()
            .unwrap()
    );
}
