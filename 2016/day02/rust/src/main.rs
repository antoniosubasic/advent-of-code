use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

struct Position {
    row: usize,
    column: usize,
}

struct Keypad {
    keypad: Vec<Vec<char>>,
    position: Position,
}

impl Keypad {
    fn move_position(&mut self, direction: char) {
        let current_row_len = self.keypad[self.position.row].len();

        match direction {
            'U' => {
                if self.position.row > 0 {
                    let above_row_len = self.keypad[self.position.row - 1].len();
                    let allowed_cell_diff =
                        (current_row_len as isize - above_row_len as isize).abs() as usize / 2;

                    if above_row_len >= current_row_len {
                        self.position.row -= 1;
                        self.position.column += allowed_cell_diff;
                    } else {
                        if self.position.column >= allowed_cell_diff
                            && self.position.column < current_row_len - allowed_cell_diff
                        {
                            self.position.row -= 1;
                            self.position.column -= allowed_cell_diff;
                        }
                    }
                }
            }
            'R' => {
                if self.position.column < current_row_len - 1 {
                    self.position.column += 1;
                }
            }
            'L' => {
                if self.position.column > 0 {
                    self.position.column -= 1;
                }
            }
            'D' => {
                if self.position.row < self.keypad.len() - 1 {
                    let below_row_len = self.keypad[self.position.row + 1].len();
                    let allowed_cell_diff =
                        (current_row_len as isize - below_row_len as isize).abs() as usize / 2;

                    if below_row_len >= current_row_len {
                        self.position.row += 1;
                        self.position.column += allowed_cell_diff;
                    } else {
                        if self.position.column >= allowed_cell_diff
                            && self.position.column <= current_row_len - 1 - allowed_cell_diff
                        {
                            self.position.row += 1;
                            self.position.column -= allowed_cell_diff;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn execute_instructions(&mut self, instructions: &str) {
        for instruction in instructions.chars() {
            self.move_position(instruction);
        }
    }

    fn get_current_key(&self) -> char {
        self.keypad[self.position.row][self.position.column]
    }
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .map(|line| line.expect("could not read line"))
        .collect();

    let mut keypads: Vec<(Keypad, String)> = vec![
        (
            Keypad {
                keypad: vec![
                    vec!['1', '2', '3'],
                    vec!['4', '5', '6'],
                    vec!['7', '8', '9'],
                ],
                position: Position { row: 1, column: 1 },
            },
            String::new(),
        ),
        (
            Keypad {
                keypad: vec![
                    vec!['1'],
                    vec!['2', '3', '4'],
                    vec!['5', '6', '7', '8', '9'],
                    vec!['A', 'B', 'C'],
                    vec!['D'],
                ],
                position: Position { row: 2, column: 0 },
            },
            String::new(),
        ),
    ];

    for instructions in &input {
        for keypad in keypads.iter_mut() {
            keypad.0.execute_instructions(instructions);
            keypad.1.push(keypad.0.get_current_key());
        }
    }

    for keypad in keypads {
        println!("{}", keypad.1);
    }
}
