use std::fs;

fn solve(grid: &mut Vec<Vec<char>>, remove: bool) -> i32 {
    let mut marked = 0;
    let mut y = 0;

    'vertical: while y < grid.len() {
        for x in 0..grid[y].len() {
            let paperrolls_in_sliced_line = |line: &Vec<char>| -> usize {
                line[x.saturating_sub(1)..=(x + 1).min(grid[y].len() - 1)]
                    .iter()
                    .filter(|char| **char == '@')
                    .count()
            };

            if grid[y][x] == '@' {
                let top = y
                    .checked_sub(1)
                    .and_then(|y| grid.get(y))
                    .map_or(0, paperrolls_in_sliced_line);

                let left = x
                    .checked_sub(1)
                    .and_then(|x| grid[y].get(x))
                    .map_or(0, |char| (*char == '@') as usize);

                let right = (x + 1 < grid[y].len())
                    .then(|| grid[y][x + 1])
                    .map_or(0, |char| (char == '@') as usize);

                let bottom = (y + 1 < grid.len())
                    .then(|| &grid[y + 1])
                    .map_or(0, paperrolls_in_sliced_line);

                if top + left + right + bottom < 4 {
                    marked += 1;

                    if remove {
                        grid[y][x] = '.';
                        y = y.saturating_sub(1);
                        continue 'vertical;
                    }
                }
            }
        }

        y += 1;
    }

    marked
}

fn main() {
    let mut input: Vec<Vec<char>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("{}\n{}", solve(&mut input, false), solve(&mut input, true));
}
