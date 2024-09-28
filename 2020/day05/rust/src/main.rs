use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    let mut seat_ids = HashSet::new();

    for boarding_pass in input {
        let mut row = 0;
        let mut col = 0;

        for (i, c) in boarding_pass.chars().enumerate() {
            if i < 7 {
                if c == 'B' {
                    row += 1 << (6 - i);
                }
            } else {
                if c == 'R' {
                    col += 1 << (2 - (i - 7));
                }
            }
        }

        seat_ids.insert(row * 8 + col);
    }

    println!("{}", seat_ids.iter().max().unwrap());

    for seat_id in 0..128 * 8 {
        if !seat_ids.contains(&seat_id)
            && seat_ids.contains(&(seat_id - 1))
            && seat_ids.contains(&(seat_id + 1))
        {
            println!("{seat_id}");
            break;
        }
    }
}
