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

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        let command = match args[1].as_str() {
            "search" => {
                let has_flag = matches!(
                    args.get(2).map(String::as_str),
                    Some("-i" | "--ignore-case")
                );

                let offset = if has_flag { 1 } else { 0 };

                if args.len() != 4 + offset {
                    return Err("Usage: search [-i] <query> <path>");
                }

                ignore_case |= has_flag;

                Command::Search {
                    query: args[2 + offset].clone(),
                    file_path: args[3 + offset].clone(),
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

    #[test]
    fn parse_short_ignore_case_flag() {
        let args = vec![
            "agent-log-grep".to_string(),
            "search".to_string(),
            "-i".to_string(),
            "ERROR".to_string(),
            "session.jsonl".to_string(),
        ];

        let config = Config::build(&args).unwrap();

        assert!(config.ignore_case);

        match config.command {
            Command::Search { query, file_path } => {
                assert_eq!(query, "ERROR");
                assert_eq!(file_path, "session.jsonl");
            }
            _ => panic!("Expected Search command"),
        }
    }

    #[test]
    fn parse_long_ignore_case_flag() {
        let args = vec![
            "agent-log-grep".to_string(),
            "search".to_string(),
            "--ignore-case".to_string(),
            "error".to_string(),
            "session.jsonl".to_string(),
        ];

        let config = Config::build(&args).unwrap();

        assert!(config.ignore_case);
    }

    #[test]
    fn reject_missing_query_with_ignore_case() {
        let args = vec![
            "agent-log-grep".to_string(),
            "search".to_string(),
            "-i".to_string(),
            "session.jsonl".to_string(),
        ];

        assert!(Config::build(&args).is_err());
    }
}
