use chrono::{NaiveDateTime, Timelike};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(PartialEq)]
enum SleepState {
    Awake,
    Asleep,
}

enum Action {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

struct Instruction {
    datetime: NaiveDateTime,
    id: Option<u16>,
    action: Action,
}

fn main() {
    let path = Path::new("../input.txt");
    let file = File::open(&path).expect("could not open file");
    let reader = BufReader::new(file);
    let mut input: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split("] ").collect();

            Instruction {
                datetime: NaiveDateTime::parse_from_str(
                    parts[0].trim_start_matches('['),
                    "%Y-%m-%d %H:%M",
                )
                .unwrap(),
                id: if parts[1].starts_with("Guard") {
                    Some(
                        parts[1]
                            .trim_start_matches("Guard #")
                            .split(' ')
                            .nth(0)
                            .unwrap()
                            .parse()
                            .unwrap(),
                    )
                } else {
                    None
                },
                action: if parts[1].starts_with("Guard") {
                    Action::BeginsShift
                } else if parts[1] == "wakes up" {
                    Action::WakesUp
                } else {
                    Action::FallsAsleep
                },
            }
        })
        .collect();

    input.sort_by(|a, b| a.datetime.cmp(&b.datetime));

    let first = input.iter().next().unwrap();
    let mut current_datetime = first.datetime;
    let mut current_guard = (
        first.id.expect("first action must be: 'begins shift'"),
        SleepState::Awake,
    );

    let mut guards: HashMap<u16, (i64, HashMap<u32, u16>)> = HashMap::new();

    for instruction in &input {
        let guard = guards
            .entry(if let Some(id) = instruction.id {
                id
            } else {
                current_guard.0
            })
            .or_insert((0, HashMap::new()));

        match instruction.action {
            Action::BeginsShift => {
                current_guard = (instruction.id.unwrap(), SleepState::Awake);
            }
            Action::WakesUp => {
                guard.0 += (instruction.datetime - current_datetime).num_minutes();
                for minute in current_datetime.minute()..instruction.datetime.minute() {
                    *guard.1.entry(minute).or_insert(0) += 1;
                }

                current_guard.1 = SleepState::Awake;
            }
            Action::FallsAsleep => {
                current_guard.1 = SleepState::Asleep;
            }
        }

        current_datetime = instruction.datetime;
    }

    let mut guards_vec: Vec<(&u16, &(i64, HashMap<u32, u16>))> = guards.iter().collect();

    {
        guards_vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

        let guard_most_asleep = guards_vec.first().unwrap();

        let mut sleep_schedule_vec: Vec<(&u32, &u16)> = guard_most_asleep.1 .1.iter().collect();
        sleep_schedule_vec.sort_by(|a, b| b.1.cmp(a.1));

        println!(
            "{}",
            (*guard_most_asleep.0 as u32) * sleep_schedule_vec.first().unwrap().0
        );
    }
    {
        guards_vec.sort_by_key(|a| a.1 .1.values().max().unwrap_or(&0));

        let guard_most_asleep_at_minute = guards_vec.last().unwrap();

        let mut sleep_schedule_vec: Vec<(&u32, &u16)> =
            guard_most_asleep_at_minute.1 .1.iter().collect();
        sleep_schedule_vec.sort_by(|a, b| b.1.cmp(a.1));

        println!(
            "{}",
            (*guard_most_asleep_at_minute.0 as u32) * sleep_schedule_vec.first().unwrap().0
        );
    }
}
