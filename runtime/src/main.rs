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

    let year: u16;
    let day: u8;
    let language: String;

    if let Some(y) = args.get(2) {
        year = y.parse().unwrap();
        day = args.get(3).expect("no day argument").parse().unwrap();
        language = args.get(4).expect("no language argument").to_string();
    } else {
        if let Some(captures) = Regex::new(r"advent-of-code\/(\d{4})\/day(\d{2})\/([a-z]+)$")
            .unwrap()
            .captures(current_dir.to_str().unwrap())
        {
            year = captures
                .get(1)
                .expect("failed to get year")
                .as_str()
                .parse()
                .unwrap();

            day = captures
                .get(2)
                .expect("failed to get day")
                .as_str()
                .parse()
                .unwrap();

            language = captures
                .get(3)
                .expect("failed to get language")
                .as_str()
                .to_string();
        } else {
            panic!("invalid arguments provided");
        }
    }

    let project = Project::new(
        home_dir
            .join("projects")
            .join("advent-of-code")
            .join(year.to_string())
            .join(format!("day{:02}", day)),
        year,
        day,
        language,
        fs::read_to_string(home_dir.join(".config/aoc/cookie")).expect("failed to read cookie"),
    );

    match args.get(1).unwrap_or(&"run".to_string()).as_str() {
        "init" => {
            project.create().await.expect("project creation failed");
        }
        "initc" => {
            project.create().await.expect("project creation failed");
            project.open().expect("opening project failed");
        }
        "run" => {
            println!(
                "{}",
                project.execute().await.expect("project execution failed")
            );
        }
        "code" => {
            project.open().expect("opening project failed");
        }
        _ => {
            panic!("invalid command");
        }
    }
}
