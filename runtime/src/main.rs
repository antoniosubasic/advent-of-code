use colored::*;
use dirs;
use std::process::{Command, Stdio};
use std::{env, fs};
use tokio;
mod api;

#[tokio::main]
async fn main() {
    let home = dirs::home_dir().unwrap();
    let args: Vec<String> = env::args().collect();

    let year_day: Vec<&str> = args[2].split('/').collect();
    let year = year_day[0].parse::<i32>().unwrap();
    let day = year_day[1].parse::<i32>().unwrap();

    let language = args[3].as_str();

    let project_directory = home.join(format!("projects/advent-of-code/{}/day{:02}", year, day));
    let language_directory = project_directory.join(language);

    let session = api::Session::new(
        fs::read_to_string(home.join("projects/advent-of-code/runtime/files/cookie")).unwrap(),
        year,
        day,
    );

    let input_file = project_directory.join("input.txt");

    if !project_directory.exists() {
        fs::create_dir_all(&project_directory).expect("failed to create directory");
    }

    if !input_file.exists() {
        let input = session.get_input_text().await.unwrap();
        fs::write(&input_file, input).expect("failed to write input file");
    }

    match args[1].as_str() {
        "init" => {
            let language_extension = match language {
                "csharp" => "cs",
                "rust" => "rs",
                "python" => "py",
                _ => panic!("invalid language"),
            };

            let file_path = project_directory.join(format!(
                "{}/{}.{}",
                language,
                match language {
                    "csharp" => "Program",
                    "rust" => "src/main",
                    "python" => "main",
                    _ => panic!("invalid language"),
                },
                language_extension
            ));

            if file_path.exists() {
                panic!("file already exists");
            }

            if language == "python" {
                fs::create_dir(project_directory.join(language))
                    .expect("failed to create directory");
            } else {
                let status = Command::new(match language {
                    "csharp" => "dotnet",
                    "rust" => "cargo",
                    _ => panic!("invalid language"),
                })
                .args(match language {
                    "csharp" => vec!["new", "console", "-o", language_directory.to_str().unwrap()],
                    "rust" => vec!["new", "-q", language_directory.to_str().unwrap()],
                    _ => panic!("invalid language"),
                })
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("failed to create project");

                if !status.success() {
                    panic!("failed to create project");
                }
            }

            fs::copy(
                home.join("projects/advent-of-code/runtime/files")
                    .join(format!("base.{}", language_extension)),
                file_path,
            )
            .expect("failed to copy file");
        }
        "run" => {
            let run_directory = match language {
                "csharp" => language_directory.clone(),
                "rust" => language_directory.join("Cargo.toml"),
                "python" => language_directory.join("main.py"),
                _ => panic!("invalid language"),
            };

            let output = Command::new(match language {
                "csharp" => "dotnet",
                "rust" => "cargo",
                "python" => "python3",
                _ => panic!("invalid language"),
            })
            .args(match language {
                "csharp" => vec!["run", "--project", run_directory.to_str().unwrap()],
                "rust" => vec![
                    "run",
                    "--quiet",
                    "--manifest-path",
                    run_directory.to_str().unwrap(),
                ],
                "python" => vec![run_directory.to_str().unwrap()],
                _ => panic!("invalid language"),
            })
            .current_dir(language_directory)
            .output()
            .expect("failed to run project");

            if !output.status.success() {
                panic!("failed to run project");
            }

            let output_string = String::from_utf8_lossy(&output.stdout);
            let parts = output_string
                .trim_end_matches('\n')
                .split('\n')
                .collect::<Vec<&str>>();

            let solution_file = project_directory.join("solution.txt");

            if !solution_file.exists() {
                let solution_part1 = session.submit_answer(1, parts[0]).await.unwrap();
                let solution_part2 = session.submit_answer(2, parts[1]).await.unwrap();

                if solution_part1 == "True" && solution_part2 == "True" {
                    fs::write(&solution_file, format!("{}\n{}", parts[0], parts[1]))
                        .expect("failed to write solution file");
                }

                println!(
                    "{}: {}",
                    if solution_part1 == "True" {
                        "part 1".green()
                    } else {
                        "part 1".red()
                    },
                    parts[0].yellow()
                );

                println!(
                    "{}: {}",
                    if solution_part2 == "True" {
                        "part 2".green()
                    } else {
                        "part 2".red()
                    },
                    parts[1].yellow()
                );
            } else {
                let solution =
                    fs::read_to_string(&solution_file).expect("failed to read solution file");
                let solution_part1 = solution.split('\n').nth(0).unwrap().replace("\r", "");
                let solution_part2 = solution.split('\n').nth(1).unwrap().replace("\r", "");

                println!(
                    "{}: {}",
                    if parts[0] == solution_part1 {
                        "part 1".green()
                    } else {
                        "part 1".red()
                    },
                    parts[0].yellow()
                );

                println!(
                    "{}: {}",
                    if parts[1] == solution_part2 {
                        "part 2".green()
                    } else {
                        "part 2".red()
                    },
                    parts[1].yellow()
                );
            }
        }
        _ => {
            panic!("invalid command");
        }
    }
}
