use crate::log::LogRecord;

pub fn search<'a>(query: &str, records: &'a Vec<LogRecord>) -> Vec<&'a LogRecord> {
    let mut results = Vec::new();

    for record in records {
        if record.raw.contains(query) {
            results.push(record);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, records: &'a Vec<LogRecord>) -> Vec<&'a LogRecord> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for record in records {
        if record.raw.to_lowercase().contains(&query) {
            results.push(record);
        }
    }

    results
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let records = vec![
            LogRecord {
                line_number: 1,
                raw: String::from("Rust:"),
            },
            LogRecord {
                line_number: 2,
                raw: String::from("safe, fast, productive."),
            },
            LogRecord {
                line_number: 3,
                raw: String::from("Pick three."),
            },
            LogRecord {
                line_number: 4,
                raw: String::from("Duct tape."),
            },
        ];

        let results = search(query, &records);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_number, 2);
        assert_eq!(results[0].raw, "safe, fast, productive.");
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";

        let records = vec![
            LogRecord {
                line_number: 1,
                raw: String::from("Rust:"),
            },
            LogRecord {
                line_number: 2,
                raw: String::from("safe, fast, productive."),
            },
            LogRecord {
                line_number: 3,
                raw: String::from("Pick three."),
            },
            LogRecord {
                line_number: 4,
                raw: String::from("Trust me."),
            },
        ];

        let results = search_case_insensitive(query, &records);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].raw, "Rust:");
        assert_eq!(results[1].raw, "Trust me.");
    }
}
