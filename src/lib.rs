pub mod cli;
mod files;
mod log;
mod search;

use cli::{Command, Config};
use files::collect_files;
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
                let records = log::stream_logs(&file)?;

                for record in records {
                    let record = record?;

                    if search::matches_query(&query, &record.raw, config.ignore_case) {
                        if is_directory {
                            println!("{}:{}: {}", file.display(), record.line_number, record.raw);
                        } else {
                            println!("{}: {}", record.line_number, record.raw);
                        }
                    }
                }
            }
        }

        Command::Show { file_path } => {
            let path = Path::new(&file_path);

            if !path.is_file() {
                return Err("show requires a file path".into());
            }

            for record in log::stream_logs(path)? {
                let record = record?;
                println!("{}: {}", record.line_number, record.raw);
            }
        }
    }

    Ok(())
}
