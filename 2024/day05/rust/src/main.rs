use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input = input.split_once("\n\n").unwrap();

    let rules: HashMap<i32, Vec<i32>> = input
        .0
        .lines()
        .map(|line| {
            let (k, v) = line.split_once('|').unwrap();
            (k.parse().unwrap(), v.parse().unwrap())
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().push(v);
            acc
        });

    let mut updates: Vec<(Vec<i32>, bool)> = input
        .1
        .lines()
        .map(|line| {
            (
                line.split(',').map(|val| val.parse().unwrap()).collect(),
                true,
            )
        })
        .collect();

    for i in 0..updates.len() {
        let update = updates.get_mut(i).unwrap();
        let mut valid = false;

        while !valid {
            valid = true;

            for j in 0..update.0.len() {
                let page = update.0.get(j).unwrap();

                if let Some(rules) = rules.get(page) {
                    if let Some(index) = update.0.iter().enumerate().take(j).find_map(|(i, val)| {
                        if rules.contains(val) {
                            Some(i)
                        } else {
                            None
                        }
                    }) {
                        update.1 = false;
                        valid = false;

                        let element = update.0.remove(index);
                        update.0.insert(j, element);
                    }
                }
            }
        }
    }

    let sum_mids = |valid: bool| {
        updates
            .iter()
            .filter(|(_, v)| if valid { *v } else { !*v })
            .map(|(update, _)| update.get(update.len() / 2).unwrap())
            .sum::<i32>()
    };

    println!("{}\n{}", sum_mids(true), sum_mids(false));
}
