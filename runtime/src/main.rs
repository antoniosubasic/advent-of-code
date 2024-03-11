use colored::*;
use std::{env, fs};
use tokio;
mod api;
mod proj;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let option = args.get(1).unwrap();
    let year = args.get(2).unwrap().parse::<u32>().unwrap();
    let day = args.get(3).unwrap().parse::<u32>().unwrap();
    let language = args.get(4).unwrap();

    let home = dirs::home_dir().unwrap();
    let cookie = fs::read_to_string(home.join(".aoc/cookie")).expect("failed to read cookie");

    let project = proj::Project::new(
        home.join("projects")
            .join("advent-of-code")
            .join(year.to_string())
            .join(format!("day{:02}", day)),
        year,
        day,
        language,
        cookie,
    );

    match option.as_str() {
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
        _ => {
            panic!("invalid command");
        }
    }
}
