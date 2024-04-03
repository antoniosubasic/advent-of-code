use regex::Regex;
use std::{env, fs};
use tokio;

mod api;
mod proj;

use crate::proj::Project;

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

    let project = Project::new(
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
        "initc" => {
            if let Err(error) = project.create().await {
                println!("{}", error);
            } else {
                if let Err(error) = project.open() {
                    println!("{}", error);
                }
            }
        }
        "run" => {
            let execution = project.execute().await;

            if let Err(error) = execution {
                println!("{}", error);
            } else {
                println!("{}", execution.unwrap());
            }
        }
        "code" => {
            if let Err(error) = project.open() {
                println!("{}", error);
            }
        }
        _ => {
            panic!("invalid command");
        }
    }
}
