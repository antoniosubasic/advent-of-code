use std::{collections::HashSet, fs};

fn react(polymer: &mut Vec<char>) {
    let mut reacted = true;

    while reacted {
        reacted = false;
        let mut i = 0;

        while i < polymer.len() - 1 {
            if polymer[i] != polymer[i + 1]
                && polymer[i].to_lowercase().eq(polymer[i + 1].to_lowercase())
            {
                polymer.remove(i);
                polymer.remove(i);
                reacted = true;
                if i > 0 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    let input: Vec<char> = fs::read_to_string("../input.txt")
        .expect("could not open file")
        .trim_end_matches('\n')
        .chars()
        .collect();

    let mut polymer = input.clone();
    react(&mut polymer);
    println!("{}", polymer.len());

    let unittypes: HashSet<char> = input.iter().map(|&c| c).collect();
    let mut min_length = usize::MAX;

    for unittype in unittypes {
        let mut polymer = input.clone();
        polymer.retain(|&c| c.to_lowercase().next().unwrap() != unittype);
        react(&mut polymer);

        if min_length > polymer.len() {
            min_length = polymer.len();
        }
    }

    println!("{}", min_length);
}
