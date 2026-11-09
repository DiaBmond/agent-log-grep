pub mod cli;
mod files;
mod log;
mod search;

use cli::{Command, Config};
use files::collect_files;
use log::read_logs;
use std::error::Error;
use std::path::Path;

pub use search::{search, search_case_insensitive};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::Search { query, file_path } => {
            let path = Path::new(&file_path);
            let is_directory = path.is_dir();

            let files = collect_files(path)?;

            for file in files {
                let records = read_logs(&file)?;

                let results = if config.ignore_case {
                    search_case_insensitive(&query, &records)
                } else {
                    search(&query, &records)
                };

                for record in results {
                    if is_directory {
                        println!("{}:{}: {}", file.display(), record.line_number, record.raw);
                    } else {
                        println!("{}: {}", record.line_number, record.raw);
                    }
                }
            }
        }

        Command::Show { file_path } => {
            let path = Path::new(&file_path);

            if !path.is_file() {
                return Err("show requires a file path".into());
            }

            let records = read_logs(path)?;

            for record in records {
                println!("{}: {}", record.line_number, record.raw);
            }
        }
    }

    Ok(())
}
