use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    let (mut part1, mut part2) = (0, 0);

    for y in 0..input.len() {
        let line: Vec<char> = input[y].chars().collect();

        for x in 0..line.len() {
            match *line.get(x).unwrap() {
                'X' => {
                    let word = "XMAS";

                    let left = x - (word.len() - 1);
                    let right = x + word.len() - 1;

                    for range in vec![left..=x, x..=right] {
                        if line
                            .get(range)
                            .map(|chars| chars.iter().collect::<String>())
                            .map_or(false, |w| {
                                w == word || w.chars().rev().collect::<String>() == word
                            })
                        {
                            part1 += 1;
                        }
                    }

                    let top = y - (word.len() - 1);
                    let bottom = y + word.len() - 1;

                    for (i, range) in vec![top..=y, y..=bottom].iter().enumerate() {
                        if let Some(mut lines) =
                            input.get(range.clone()).map(|lines| lines.to_vec())
                        {
                            if i == 0 {
                                lines.reverse();
                            }

                            let mut strings = vec![String::new(); 3];

                            for y in 0..word.len() {
                                let line: Vec<char> = lines.get(y).unwrap().chars().collect();

                                strings[0].push(*line.get(x).unwrap());

                                if let Some(left) = line.get(x - y) {
                                    strings[1].push(*left);
                                }

                                if let Some(right) = line.get(x + y) {
                                    strings[2].push(*right);
                                }
                            }

                            part1 += strings
                                .iter()
                                .filter(|&w| {
                                    w == word || w.chars().rev().collect::<String>() == word
                                })
                                .count();
                        }
                    }
                }
                'A' => {
                    let word = "MAS";

                    if let Some(lines) = input.get((y - 1)..=(y + 1)).map(|lines| lines.to_vec()) {
                        let mut strings = vec![String::new(); 2];

                        for y in 0..word.len() {
                            let line: Vec<char> = lines.get(y).unwrap().chars().collect();

                            if let Some(&left) = line.get(x - ((word.len() - 1) / 2) + y) {
                                strings[0].push(left);
                            }

                            if let Some(&right) = line.get(x + ((word.len() - 1) / 2) - y) {
                                strings[1].push(right);
                            }
                        }

                        if strings
                            .iter()
                            .all(|w| w == word || w.chars().rev().collect::<String>() == word)
                        {
                            part2 += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!("{}\n{}", part1, part2);
}
