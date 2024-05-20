use anyhow::{Error, Result};
use colored::*;
use dirs;
use std::fs;
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};

use crate::api::Session;
use crate::{Args, Command};

pub struct Puzzle {
    pub args: Args,
    pub session: Session,
    pub config_directory: PathBuf,
    pub puzzle_directory: PathBuf,
    pub full_directory: PathBuf,
    pub command: Vec<String>,
}

impl Puzzle {
    pub fn new(args: &Args) -> Result<Self> {
        let home = dirs::home_dir().ok_or(Error::msg("could not find home directory"))?;

        let config_directory = home.join(".config/aoc");
        let cookie_file = config_directory.join("cookie");
        let project_directory = home
            .join("projects/advent-of-code")
            .join(args.puzzle.year.to_string())
            .join(format!("day{:02}", &args.puzzle.day));
        let full_directory = project_directory.join(format!("{}", args.puzzle.language));

        match args.command {
            Command::Init if full_directory.exists() => Err(Error::msg("puzzle already exists")),
            Command::Run | Command::Open if !full_directory.exists() => {
                Err(Error::msg("puzzle does not exist"))
            }
            _ => {
                let session = Session::new(fs::read_to_string(cookie_file)?);

                let command = match args.command {
                    Command::Init => args.puzzle.language.init(&full_directory),
                    Command::Run => args.puzzle.language.run(&full_directory),
                    Command::Open => args.puzzle.language.open(),
                };

                Ok(Self {
                    args: args.clone(),
                    session,
                    config_directory,
                    puzzle_directory: project_directory,
                    full_directory,
                    command,
                })
            }
        }
    }

    pub async fn execute(&self) -> Result<Option<String>> {
        self.session
            .ensure_input(&self.puzzle_directory.join("input.txt"), &self.args.puzzle)
            .await?;

        match self.args.command {
            Command::Run => {
                if let Some(compile_command) =
                    self.args.puzzle.language.compile(&self.full_directory)
                {
                    let command = ProcessCommand::new(&compile_command[0])
                        .args(&compile_command[1..])
                        .current_dir(&self.full_directory)
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .spawn();

                    if let Err(_) = command {
                        return Err(Error::msg("failed to compile"));
                    }
                }

                let command = ProcessCommand::new("time")
                    .arg("-p")
                    .args(&self.command)
                    .current_dir(&self.full_directory)
                    .output()?;

                let stdout = String::from_utf8(command.stdout)?;
                let stderr = String::from_utf8(command.stderr)?;

                if !command.status.success() {
                    Err(Error::msg(stderr))
                } else {
                    let mut lines = stdout.lines();

                    let part1 = lines.next().ok_or(Error::msg("part 1 not found"))?;
                    let part2 = lines.next().ok_or(Error::msg("part 2 not found"))?;

                    let solution = self
                        .session
                        .evaluate_solution(
                            &self.puzzle_directory.join(".solution.txt"),
                            self.args.puzzle.year,
                            self.args.puzzle.day,
                            (part1, part2),
                        )
                        .await?;

                    let lines = stderr.lines().collect::<Vec<&str>>();

                    let execution_time = std::time::Duration::from_secs_f32(
                        lines[lines.len() - 3]
                            .split(' ')
                            .nth(1)
                            .unwrap()
                            .parse::<f32>()
                            .unwrap(),
                    );

                    let minutes = execution_time.as_secs() / 60;
                    let seconds = execution_time.as_millis() as f64 / 1000.0;

                    Ok(Some(format!(
                        "{}\n{}\n\n{}",
                        if solution.0 {
                            part1.green()
                        } else {
                            part1.red()
                        },
                        if solution.1 {
                            part2.green()
                        } else {
                            part2.red()
                        },
                        if minutes > 0 {
                            format!("{}m {:.3}s", minutes, seconds)
                        } else {
                            format!("{:.3}s", seconds)
                        }
                    )))
                }
            }
            _ => {
                let mut child = ProcessCommand::new(&self.command[0])
                    .args(&self.command[1..])
                    .spawn()?;

                child.wait()?;

                match self.args.command {
                    Command::Init => {
                        let base_file = self
                            .config_directory
                            .join(format!("base.{}", self.args.puzzle.language.extension()));

                        let code_file = self.args.puzzle.language.code_file(&self.full_directory);

                        fs::copy(base_file, code_file)?;
                    }
                    _ => {}
                }

                Ok(None)
            }
        }
    }
}
