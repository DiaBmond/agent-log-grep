pub mod cli;
mod log;
mod search;

use cli::{Command, Config};
use log::read_logs;
use std::error::Error;

pub use search::{search, search_case_insensitive};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::Search { query, file_path } => {
            let records = read_logs(&file_path)?;

            let results = if config.ignore_case {
                search_case_insensitive(&query, &records)
            } else {
                search(&query, &records)
            };

            for line in results {
                println!("{}: {}", line.line_number, line.raw);
            }
        }

        Command::Show { file_path } => {
            let records = read_logs(&file_path)?;
            for line in records {
                println!("{}: {}", line.line_number, line.raw);
            }
        }
    }

    Ok(())
}
