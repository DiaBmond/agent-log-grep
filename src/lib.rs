pub mod cli;
mod search;

use cli::{Command, Config};
use std::error::Error;
use std::fs;

pub use search::{search, search_case_insensitive};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
