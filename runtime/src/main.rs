use colored::*;
use regex::Regex;
use std::{env, fs};
use tokio;
mod api;
mod proj;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let home_dir = dirs::home_dir().unwrap();
    let current_dir = env::current_dir().unwrap();

    let command = args.get(1).map(|s| s.clone()).unwrap_or("run".to_string());

    let mut year = args
        .get(2)
        .unwrap_or(&"0".to_string())
        .parse::<u16>()
        .unwrap();
    let mut day = args
        .get(3)
        .unwrap_or(&"0".to_string())
        .parse::<u8>()
        .unwrap();
    let mut language = args.get(4).map(|s| s.clone()).unwrap_or("0".to_string());

    if year == 0 {
        if let Some(captures) = Regex::new(r"advent-of-code\/(\d{4})\/day(\d{2})\/([a-z]+)$")
            .unwrap()
            .captures(current_dir.to_str().unwrap())
        {
            year = captures.get(1).unwrap().as_str().parse().unwrap();
            day = captures.get(2).unwrap().as_str().parse().unwrap();
            language = captures.get(3).unwrap().as_str().to_string();
        } else {
            panic!("invalid arguments provided");
        }
    }

    let cookie = fs::read_to_string(home_dir.join(".aoc/cookie")).expect("failed to read cookie");

    let project = proj::Project::new(
        home_dir
            .join("projects")
            .join("advent-of-code")
            .join(year.to_string())
            .join(format!("day{:02}", day)),
        year,
        day,
        language,
        cookie,
    );

    match command.as_str() {
        "init" => {
            if let Err(error) = project.create().await {
                println!("{}", error);
            }
        }
        "run" => {
            let execution_result = project.execute().await;

            if let Err(error) = execution_result {
                println!("{}", error);
            } else {
                let (part1, part2) = execution_result.unwrap();

                println!(
                    "{}: {}",
                    if part1.0 {
                        "part 1".green()
                    } else {
                        "part 1".red()
                    },
                    part1.1
                );

                println!(
                    "{}: {}",
                    if part2.0 {
                        "part 2".green()
                    } else {
                        "part 2".red()
                    },
                    part2.1
                );
            }
        }
        "code" => {
            if let Err(error) = project.open() {
                println!("{}", error);
            }
        }
        "initc" => {
            if let Err(error) = project.create().await {
                println!("{}", error);
            } else {
                if let Err(error) = project.open() {
                    println!("{}", error);
                }
            }
        }
        _ => {
            panic!("invalid command");
        }
    }
}
