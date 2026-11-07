use std::env;

pub struct Config {
    pub command: Command,
    pub ignore_case: bool,
}

pub enum Command {
    Search { query: String, file_path: String },
    Show { file_path: String },
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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
