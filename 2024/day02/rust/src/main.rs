use std::fs;

fn evaluate_report(report: &Vec<i32>, removable: bool) -> bool {
    let mut invalids = 0;

    let ascending = report[1] > report[0];

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if !(1..=3).contains(&diff.abs()) || ascending && diff < 0 || !ascending && diff > 0 {
            invalids += 1;
        }
    }

    match invalids {
        0 => true,
        _ => {
            removable && {
                for i in 0..report.len() {
                    let mut report = report.clone();
                    report.remove(i);
                    if evaluate_report(&report, false) {
                        return true;
                    }
                }

                false
            }
        }
    }
}

fn main() {
    let input: Vec<Vec<i32>> = fs::read_to_string("../input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect();

    let count_reports = |removable| {
        input
            .iter()
            .filter(|report| evaluate_report(report, removable))
            .count()
    };

    println!("{}\n{}", count_reports(false), count_reports(true));
}
