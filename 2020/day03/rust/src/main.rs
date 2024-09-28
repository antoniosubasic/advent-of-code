use std::fs;

struct Slope {
    trees_found: u32,
    position: (usize, usize),
    diff: (usize, usize),
}

impl Slope {
    fn new(x_diff: usize, y_diff: usize) -> Self {
        Self {
            trees_found: 0,
            position: (0, 0),
            diff: (x_diff, y_diff),
        }
    }

    fn advance(&mut self) {
        self.position.0 += self.diff.0;
        self.position.1 += self.diff.1;
    }
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    for slope in &mut slopes {
        while slope.position.1 < input.len() {
            if input[slope.position.1][slope.position.0 % input[0].len()] == '#' {
                slope.trees_found += 1;
            }

            slope.advance();
        }
    }

    println!(
        "{}\n{}",
        slopes[1].trees_found,
        slopes.iter().fold(1, |acc, slope| acc * slope.trees_found)
    );
}
