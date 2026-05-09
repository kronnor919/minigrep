use std::env;
use std::process;

use minigrep::handle_options;
use minigrep::{AppError, Config, run};

fn main() {
    // Read arguments
    let args: Vec<String> = env::args().collect();

    // Extract options (--opt)
    let options = &args.iter().filter(|arg| arg.starts_with("--")).collect();
    handle_options(options);

    // Build config                          Pass args without --
    let config = match Config::build(&args[options.len() + 1..]) {
        Ok(cfg) => cfg,

        Err(AppError::MissingArgs()) => {
            eprintln!("minigrep takes 2 arguments: [QUERY] [FILEPATH]");
            eprintln!("Try 'minigrep --help' for more information");
            process::exit(1);
        }

        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    // Run search
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
