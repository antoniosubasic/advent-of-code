use std::{fs, ops::RangeInclusive};

fn main() {
    let mut input: (Vec<RangeInclusive<u64>>, Vec<u64>) = fs::read_to_string("../input.txt")
        .unwrap()
        .split_once("\n\n")
        .map(|(ranges, ids)| {
            (
                ranges
                    .lines()
                    .map(|line| {
                        line.split_once('-')
                            .map(|(from, to)| from.parse().unwrap()..=to.parse().unwrap())
                            .unwrap()
                    })
                    .collect(),
                ids.lines().map(|line| line.parse().unwrap()).collect(),
            )
        })
        .unwrap();

    input.0.sort_unstable_by_key(|range| *range.start());

    let mut i = 1;
    while i < input.0.len() {
        let (previous, current) = (&input.0[i - 1], &input.0[i]);

        if previous.end() >= current.start() {
            let new_range = *previous.start()..=(*previous.end().max(current.end()));
            input.0.splice((i - 1)..=i, [new_range]);
        } else {
            i += 1;
        }
    }

    println!(
        "{}\n{}",
        input
            .1
            .iter()
            .filter(|id| input.0.iter().any(|range| range.contains(id)))
            .count(),
        input
            .0
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum::<u64>()
    );
}
