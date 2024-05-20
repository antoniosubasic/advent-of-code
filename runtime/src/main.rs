use std::path::PathBuf;

use clap::{Parser, ValueEnum};

mod api;
mod puzzle;

#[derive(Parser, Clone)]
#[clap(rename_all = "kebab-case")]
struct Args {
    #[clap(subcommand)]
    command: Command,
    #[clap(flatten)]
    puzzle: PuzzleData,
}

#[derive(Parser, Clone)]
pub enum Command {
    Init,
    Run,
    Open,
}

#[derive(Parser, Clone)]
pub struct PuzzleData {
    #[clap(short = 'd', long)]
    day: u8,
    #[clap(short = 'y', long)]
    year: u16,
    #[clap(short = 'l', long)]
    language: Language,
}

#[derive(Parser, Clone, ValueEnum)]
pub enum Language {
    Rust,
    Python,
    Csharp,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Python => write!(f, "python"),
            Language::Csharp => write!(f, "csharp"),
        }
    }
}

impl Language {
    fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::Csharp => "cs",
        }
    }

    fn code_file(&self, directory: &PathBuf) -> PathBuf {
        let relative_path = match self {
            Language::Rust => PathBuf::from("src/main"),
            Language::Python => PathBuf::from("main"),
            Language::Csharp => PathBuf::from("Program"),
        };

        directory.join(format!(
            "{}.{}",
            relative_path.to_str().unwrap(),
            self.extension()
        ))
    }

    fn command(&self) -> &'static str {
        match self {
            Language::Rust => "cargo",
            Language::Python => "python3",
            Language::Csharp => "dotnet",
        }
    }

    fn init_args(&self) -> Option<Vec<&'static str>> {
        match self {
            Language::Rust => Some(vec!["new", "-q", "--bin"]),
            Language::Python => None,
            Language::Csharp => Some(vec!["new", "console", "-o"]),
        }
    }

    fn compile_args(&self) -> Option<Vec<&'static str>> {
        match self {
            Language::Rust => Some(vec!["build", "--release", "--manifest-path"]),
            Language::Python => None,
            Language::Csharp => Some(vec!["build"]),
        }
    }

    fn run_args(&self) -> Vec<&'static str> {
        match self {
            Language::Rust => vec!["run", "-q", "--release", "--manifest-path"],
            Language::Python => vec![],
            Language::Csharp => vec!["run", "--project"],
        }
    }

    pub fn init(&self, directory: &PathBuf) -> Vec<String> {
        let mut args = if let Some(mut args) = self.init_args() {
            args.insert(0, self.command());
            args
        } else {
            vec!["mkdir", "-p"]
        }
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();

        args.push(directory.to_str().unwrap().to_string());
        args
    }

    pub fn compile(&self, directory: &PathBuf) -> Option<Vec<String>> {
        if let Some(args) = self.compile_args() {
            let mut args = args.iter().map(|&s| s.to_string()).collect::<Vec<String>>();

            args.insert(0, self.command().to_string());
            args.push(
                match self {
                    Language::Rust => directory.join("Cargo.toml"),
                    _ => PathBuf::new(),
                }
                .to_str()
                .unwrap()
                .to_string(),
            );

            Some(args)
        } else {
            None
        }
    }

    pub fn run(&self, directory: &PathBuf) -> Vec<String> {
        let mut args = self
            .run_args()
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        args.insert(0, self.command().to_string());
        args.push(
            match self {
                Language::Rust => directory.join("Cargo.toml"),
                Language::Python => directory.join("main.py"),
                Language::Csharp => directory.clone(),
            }
            .to_str()
            .unwrap()
            .to_string(),
        );

        args
    }

    pub fn open(&self) -> Vec<String> {
        match self {
            Language::Rust => vec!["code"],
            Language::Python => vec!["code"],
            Language::Csharp => vec!["code"],
        }
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>()
    }
}

#[tokio::main]
async fn main() {
    match puzzle::Puzzle::new(&Args::parse()) {
        Ok(puzzle) => match puzzle.execute().await {
            Ok(Some(output)) => println!("{output}"),
            Err(error) => eprintln!("{error}"),
            Ok(None) => {}
        },
        Err(error) => eprintln!("{error}"),
    }
}
