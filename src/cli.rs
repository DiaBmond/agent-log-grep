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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_search_command() {
        let args = vec![
            "agent-log-grep".to_string(),
            "search".to_string(),
            "error".to_string(),
            "log.jsonl".to_string(),
        ];

        let config = Config::build(&args).unwrap();

        match config.command {
            Command::Search { query, file_path } => {
                assert_eq!(query, "error");
                assert_eq!(file_path, "log.jsonl");
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn parse_show_command() {
        let args = vec![
            "agent-log-grep".to_string(),
            "show".to_string(),
            "log.jsonl".to_string(),
        ];

        let config = Config::build(&args).unwrap();

        match config.command {
            Command::Show { file_path } => {
                assert_eq!(file_path, "log.jsonl");
            }
            _ => panic!("Expected Show command"),
        }
    }

    #[test]
    fn reject_missing_command() {
        let args = vec!["agent-log-grep".to_string()];

        assert!(Config::build(&args).is_err());
    }

    #[test]
    fn reject_unknown_command() {
        let args = vec![
            "agent-log-grep".to_string(),
            "delete".to_string(),
            "log.jsonl".to_string(),
        ];

        assert!(Config::build(&args).is_err());
    }

    #[test]
    fn reject_invalid_search_arguments() {
        let args = vec![
            "agent-log-grep".to_string(),
            "search".to_string(),
            "error".to_string(),
        ];

        assert!(Config::build(&args).is_err());
    }

    #[test]
    fn reject_extra_show_arguments() {
        let args = vec![
            "agent-log-grep".to_string(),
            "show".to_string(),
            "a.jsonl".to_string(),
            "b.jsonl".to_string(),
        ];

        assert!(Config::build(&args).is_err());
    }
}
