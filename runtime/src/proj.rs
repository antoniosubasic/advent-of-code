use colored::*;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::api::Session;

pub struct Language {
    pub name: String,
    pub extension: String,
    pub command: String,
    pub directory: PathBuf,
}

impl Language {
    pub fn new(name: String, directory: &PathBuf) -> Self {
        let name_cloned = name.clone();

        Self {
            name,
            extension: match name_cloned.as_str() {
                "csharp" => "cs",
                "rust" => "rs",
                "python" => "py",
                _ => panic!("invalid language"),
            }
            .to_string(),
            command: match name_cloned.as_str() {
                "csharp" => "dotnet",
                "rust" => "cargo",
                "python" => "python3",
                _ => panic!("invalid language"),
            }
            .to_string(),
            directory: directory.join(name_cloned),
        }
    }

    pub fn get_program_file(&self) -> String {
        format!(
            "{}.{}",
            match self.name.as_str() {
                "csharp" => "Program",
                "rust" => "src/main",
                "python" => "main",
                _ => panic!("invalid language"),
            },
            &self.extension
        )
    }

    pub fn init_command(&self) -> Vec<&str> {
        let arguments = match self.name.as_str() {
            "csharp" => Some(vec![
                "new",
                "console",
                "-o",
                &self.directory.to_str().unwrap(),
            ]),
            "rust" => Some(vec!["new", "-q", &self.directory.to_str().unwrap()]),
            "python" => None,
            _ => panic!("invalid language"),
        };

        if let Some(mut arguments) = arguments {
            arguments.insert(0, &self.command);
            arguments
        } else {
            vec!["mkdir", "-p", &self.directory.to_str().unwrap()]
        }
    }

    pub fn compile_command(&self) -> Option<Vec<String>> {
        let rust_dir = self.directory.join("Cargo.toml");

        let arguments = match self.name.as_str() {
            "rust" => Some(vec![
                "build",
                "--release",
                "--manifest-path",
                &rust_dir.to_str().unwrap(),
            ]),
            "csharp" => None,
            "python" => None,
            _ => panic!("invalid language"),
        };

        if let Some(mut arguments) = arguments {
            arguments.insert(0, &self.command);
            Some(arguments.into_iter().map(|s| s.to_string()).collect())
        } else {
            None
        }
    }

    pub fn run_command(&self) -> Vec<String> {
        let rust_dir = self.directory.join("Cargo.toml");
        let python_dir = self.directory.join("main.py");

        let mut arguments = match self.name.as_str() {
            "csharp" => vec!["run", "--project", &self.directory.to_str().unwrap()],
            "rust" => vec![
                "run",
                "--release",
                "--manifest-path",
                rust_dir.to_str().unwrap(),
            ],
            "python" => vec![python_dir.to_str().unwrap()],
            _ => panic!("invalid language"),
        };

        arguments.insert(0, &self.command);
        arguments.into_iter().map(|s| s.to_string()).collect()
    }
}

pub struct Project {
    pub year: u16,
    pub day: u8,
    pub language: Language,
    pub directory: PathBuf,
    session: Session,
}

impl Project {
    pub fn new(directory: PathBuf, year: u16, day: u8, language: String, cookie: String) -> Self {
        Self {
            year,
            day,
            language: Language::new(language, &directory),
            directory,
            session: Session::new(cookie),
        }
    }

    pub async fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home = dirs::home_dir().unwrap();

        if self.language.directory.exists() {
            Err("project already exists".into())
        } else {
            fs::create_dir_all(&self.directory).unwrap();

            let input = self.session.get_input_text(self.year, self.day).await?;
            fs::write(self.directory.join("input.txt"), input).unwrap();

            let base_file = home
                .join(".config/aoc")
                .join(format!("base.{}", self.language.extension));

            let init_command = self.language.init_command();

            let status = Command::new(&init_command[0])
                .args(&init_command[1..])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;

            if !status.success() {
                return Err("failed to create project".into());
            }

            std::fs::copy(
                base_file,
                &self
                    .language
                    .directory
                    .join(self.language.get_program_file()),
            )?;

            Ok(())
        }
    }

    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let input_file = self.directory.join("input.txt");

        if !input_file.exists() {
            let input = self.session.get_input_text(self.year, self.day).await?;
            fs::write(&input_file, input)?;
        }

        if let Some(compile_command) = self.language.compile_command() {
            let compile_command = Command::new(&compile_command[0])
                .args(&compile_command[1..])
                .current_dir(&self.language.directory)
                .output()?;

            if !compile_command.status.success() {
                return Err(format!(
                    "failed to compile project: {}",
                    String::from_utf8_lossy(&compile_command.stderr)
                )
                .into());
            }
        }

        let run_command = self.language.run_command();

        let output = Command::new("time")
            .arg("-p")
            .args(run_command)
            .current_dir(&self.language.directory)
            .output()?;

        if !output.status.success() {
            Err(format!(
                "failed to run project: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into())
        } else {
            let run_command_stdout = String::from_utf8_lossy(&output.stdout);
            let part1 = run_command_stdout.lines().nth(0).unwrap();
            let part2 = run_command_stdout.lines().nth(1).unwrap();

            let solution_file = self.directory.join("solution.txt");
            let part1_correct: bool;
            let part2_correct: bool;

            if !solution_file.exists() {
                let part1_correct_result = self
                    .session
                    .submit_answer(self.year, self.day, 1, &part1)
                    .await;

                if let Err(part1_error) = part1_correct_result {
                    return Err(part1_error.into());
                } else {
                    part1_correct = part1_correct_result.unwrap();
                }

                let part2_correct_result = self
                    .session
                    .submit_answer(self.year, self.day, 2, &part2)
                    .await;

                if let Err(part2_error) = part2_correct_result {
                    return Err(part2_error.into());
                } else {
                    part2_correct = part2_correct_result.unwrap();
                }

                if part1_correct && part2_correct {
                    std::fs::write(&solution_file, format!("{}\n{}", part1, part2))?;
                }
            } else {
                let solution = std::fs::read_to_string(&solution_file)?;

                part1_correct = part1 == solution.lines().nth(0).unwrap();
                part2_correct = part2 == solution.lines().nth(1).unwrap();
            }

            let run_command_stderr = String::from_utf8_lossy(&output.stderr);
            let execution_time = std::time::Duration::from_secs_f32(
                run_command_stderr
                    .lines()
                    .nth(0)
                    .unwrap()
                    .split(' ')
                    .nth(1)
                    .unwrap()
                    .parse::<f32>()
                    .unwrap(),
            );

            let minutes = execution_time.as_secs() / 60;
            let seconds = execution_time.as_millis() as f64 / 1000.0;

            Ok(format!(
                "{}\n{}\n\n{}",
                if part1_correct {
                    part1.green()
                } else {
                    part1.red()
                },
                if part2_correct {
                    part2.green()
                } else {
                    part2.red()
                },
                if minutes == 0 {
                    format!("{:.2}s", seconds)
                } else {
                    format!("{}m{:.2}s", minutes, seconds)
                }
            ))
        }
    }

    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.language.directory.exists() {
            Err("project does not exist".into())
        } else {
            let status = Command::new("code")
                .arg(&self.language.directory)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;

            if !status.success() {
                Err("failed to open project".into())
            } else {
                Ok(())
            }
        }
    }
}
