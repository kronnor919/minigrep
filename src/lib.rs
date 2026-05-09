use std::env;
use std::fs;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing arguments")]
    MissingArgs(),

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, AppError> {
        if args.len() < 3 {
            return Err(AppError::MissingArgs());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search(query: String, content_lines: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in content_lines {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive(query: String, content_lines: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in content_lines {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

fn read_lines(file_path: String) -> Result<Vec<String>, io::Error> {
    let content_lines: Vec<String> = match fs::read_to_string(file_path) {
        Ok(content) => content.lines().map(String::from).collect(),
        Err(e) => return Err(e),
    };

    Ok(content_lines)
}

pub fn run(config: Config) -> Result<Vec<String>, AppError> {
    let file_lines = read_lines(config.file_path)?;

    let results: Vec<String> = match config.ignore_case {
        true => search_case_insensitive(config.query, file_lines),
        false => search(config.query, file_lines),
    };

    Ok(results)
}
