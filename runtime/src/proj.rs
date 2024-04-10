use colored::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::api::Session;

pub struct Language {
    pub name: String,
    pub extension: String,
}

impl Language {
    pub fn new(name: String) -> Self {
        let extension = match name.as_str() {
            "csharp" => "cs",
            "rust" => "rs",
            "python" => "py",
            _ => panic!("invalid language"),
        };

        Self {
            name: name.to_string(),
            extension: extension.to_string(),
        }
    }

    pub fn get_base_file(&self) -> String {
        format!("base.{}", self.extension)
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
            self.extension
        )
        .to_owned()
    }

    pub fn get_command(&self) -> &str {
        match self.name.as_str() {
            "csharp" => "dotnet",
            "rust" => "cargo",
            "python" => "python3",
            _ => panic!("invalid language"),
        }
    }

    pub fn has_init_command(&self) -> bool {
        self.name != "python"
    }

    pub fn get_init_arguments(&self, directory: &Path) -> Vec<String> {
        match self.name.as_str() {
            "csharp" => vec![
                "new".to_string(),
                "console".to_string(),
                "-o".to_string(),
                directory.to_str().unwrap().to_string(),
            ],
            "rust" => vec![
                "new".to_string(),
                "-q".to_string(),
                directory.to_str().unwrap().to_string(),
            ],
            _ => panic!("invalid language"),
        }
    }

    pub fn has_compile_command(&self) -> bool {
        self.name == "rust"
    }

    pub fn get_compile_arguments(&self) -> Vec<String> {
        match self.name.as_str() {
            "rust" => vec!["build".to_string(), "--release".to_string()],
            _ => panic!("invalid language"),
        }
    }

    pub fn get_run_arguments(&self, directory: &Path) -> Vec<String> {
        match self.name.as_str() {
            "csharp" => vec![
                "run".to_string(),
                "--project".to_string(),
                directory.join("csharp").to_str().unwrap().to_string(),
            ],
            "rust" => vec![
                "run".to_string(),
                "--release".to_string(),
                "--quiet".to_string(),
                "--manifest-path".to_string(),
                directory
                    .join("rust/Cargo.toml")
                    .to_str()
                    .unwrap()
                    .to_string(),
            ],
            "python" => vec![directory
                .join("python/main.py")
                .to_str()
                .unwrap()
                .to_string()],
            _ => panic!("invalid language"),
        }
    }
}

pub struct Project {
    pub year: u16,
    pub day: u8,
    language: Language,
    pub directory: PathBuf,
    session: Session,
}

impl Project {
    pub fn new(directory: PathBuf, year: u16, day: u8, language: String, cookie: String) -> Self {
        Self {
            year,
            day,
            language: Language::new(language),
            directory,
            session: Session::new(cookie),
        }
    }

    pub async fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home = dirs::home_dir().unwrap();
        let language_directory = &self.directory.join(&self.language.name);

        if language_directory.exists() {
            Err("project already exists".into())
        } else {
            fs::create_dir_all(&self.directory).unwrap();

            let input = self.session.get_input_text(self.year, self.day).await?;
            fs::write(self.directory.join("input.txt"), input).unwrap();

            let base_file = home.join(".config/aoc").join(self.language.get_base_file());

            if self.language.has_init_command() {
                let status = Command::new(self.language.get_command())
                    .args(self.language.get_init_arguments(language_directory))
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()?;

                if !status.success() {
                    return Err("failed to create project".into());
                }
            }

            if !language_directory.exists() {
                fs::create_dir_all(language_directory)?;
            }

            std::fs::copy(
                base_file,
                language_directory.join(self.language.get_program_file()),
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

        if self.language.has_compile_command() {
            let compile_command = Command::new(self.language.get_command())
                .args(self.language.get_compile_arguments())
                .current_dir(self.directory.join(&self.language.name))
                .output()?;

            if !compile_command.status.success() {
                return Err(format!(
                    "failed to compile project: {}",
                    String::from_utf8_lossy(&compile_command.stderr)
                )
                .into());
            }
        }

        let run_command = Command::new("time")
            .arg("-p")
            .arg(self.language.get_command())
            .args(self.language.get_run_arguments(&self.directory))
            .current_dir(self.directory.join(&self.language.name))
            .output()?;

        if !run_command.status.success() {
            Err(format!("failed to run project: {}", String::from_utf8_lossy(&run_command.stderr)).into())
        } else {
            let run_command_output = String::from_utf8_lossy(&run_command.stdout);
            let part1 = run_command_output.lines().nth(0).unwrap().to_string();
            let part2 = run_command_output.lines().nth(1).unwrap().to_string();

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

            let execution_time_string = String::from_utf8_lossy(&run_command.stderr);
            let execution_time: Vec<&str> = execution_time_string.lines().nth(2).unwrap().split(' ').nth(1).unwrap().split(':').collect();

            Ok(format!(
                "{}\n{}\n\n{}",
                if part1_correct { part1.green() } else { part1.red() },
                if part2_correct { part2.green() } else { part2.red() },
                format!("{}h {}m {}s", execution_time.get(2).unwrap_or(&"0"), execution_time.get(1).unwrap_or(&"0"), execution_time.get(0).unwrap_or(&"0"))
            ))
        }
    }

    pub fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        let language_directory = &self.directory.join(&self.language.name);

        if !language_directory.exists() {
            Err("project does not exist".into())
        } else {
            let status = Command::new("code")
                .arg(&self.directory.join(&self.language.name))
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
