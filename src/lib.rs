use std::env;
use std::fs;
use std::io;
use std::process;
use thiserror::Error;

pub const VERSION: &str = "1.1.0";
pub const GREEN: &str = "\x1b[0;32m";
pub const NC: &str = "\x1b[0m"; // No Color

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
    pub fn build(args: &[String]) -> Result<Config, AppError> {
        // [query] [filepath]
        if args.len() < 2 {
            return Err(AppError::MissingArgs());
        }

        let query = args[0].clone();
        let file_path = args[1].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search(query: &String, content_lines: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in content_lines {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive(query: &String, content_lines: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let query = query.to_lowercase();

    for line in content_lines {
        if line.to_lowercase().contains(&query) {
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

fn format_result(mut results: Vec<String>, query: &String) -> Vec<String> {
    for line in results.iter_mut() {
        if let Some(start_idx) = line.find(query) {
        let end_idx = start_idx + query.len();
        
        line.insert_str(end_idx, NC);
        line.insert_str(start_idx, GREEN);
    }
    }

    results
}

pub fn run(config: Config) -> Result<Vec<String>, AppError> {
    let file_lines = read_lines(config.file_path)?;

    let results: Vec<String> = match config.ignore_case {
        true => search_case_insensitive(&config.query, file_lines),
        false => search(&config.query, file_lines),
    };

    Ok(format_result(results, &config.query))
}

fn print_help() {
    println!("Usage: minigrep [OPTIONS] QUERY FILEPATH");
    println!("Options:");
    println!("    --help         Show this help text");
    println!("    --version      Show minigrep's version");
}

pub fn handle_options(ops: &Vec<&String>) -> () {
    for op in ops {
        match op.as_str() {
            "--help" => {
                print_help();
                process::exit(0);
            }

            "--version" => {
                println!("Minigrep v{VERSION}");
                process::exit(0);
            }

            _ => {
                eprintln!("Unrecognized option: {op}");
                process::exit(1);
            }
        }
    }
}
