use std::env;
use std::process;

use minigrep::{AppError, Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Ok(r) => r,
        Err(AppError::MissingArgs()) => {
            eprintln!("minigrep takes 2 arguments: [QUERY] [FILEPATH]");
            process::exit(1);
        }

        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    match run(config) {
        Ok(res) => {
            for line in res {
                println!("{line}");
            }
            process::exit(0);
        }
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
