use std::fs;

fn main() {
    let input: Vec<(char, u16)> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (direction, value) = line.split_at(1);
            (
                direction.chars().next().unwrap(),
                value.parse::<u16>().unwrap(),
            )
        })
        .collect();

    let mut current_dial_value: u16 = 50;
    let (mut points_at_0, mut runs_over_0) = (0, 0);

    for (direction, mut value) in input {
        let old_dial_value = current_dial_value;

        runs_over_0 += value / 100 - (if value % 100 == 0 { 1 } else { 0 });
        value %= 100;

        match direction {
            'R' => current_dial_value += value,
            'L' => current_dial_value += 100 - value,
            _ => unreachable!(),
        }

        current_dial_value %= 100;

        if current_dial_value == 0 {
            points_at_0 += 1;
        } else if old_dial_value != 0
            && (direction == 'R' && current_dial_value < old_dial_value
                || direction == 'L' && current_dial_value > old_dial_value)
        {
            runs_over_0 += 1;
        }
    }

    println!("{}\n{}", points_at_0, points_at_0 + runs_over_0);
}
