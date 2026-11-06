use agent_log_grep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub command: Command,
    pub ignore_case: bool,
}

pub enum Command {
    Search { query: String, file_path: String },
    Show { file_path: String },
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("missing command");
        }

        let command = match args[1].as_str() {
            "search" => {
                if args.len() != 4 {
                    return Err("Usage: search <query> <file_path>");
                }

                Command::Search {
                    query: args[2].clone(),
                    file_path: args[3].clone(),
                }
            }

            "show" => {
                if args.len() != 3 {
                    return Err("Usage: show <file_path>");
                }

                Command::Show {
                    file_path: args[2].clone(),
                }
            }

            _ => return Err("unknown command: use search or show"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            command,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::Search { query, file_path } => {
            let contents = fs::read_to_string(file_path)?;

            let results = if config.ignore_case {
                search_case_insensitive(&query, &contents)
            } else {
                search(&query, &contents)
            };

            for line in results {
                println!("{line}");
            }
        }

        Command::Show { file_path } => {
            let contents = fs::read_to_string(file_path)?;
            print!("{contents}");
        }
    }

    Ok(())
}
