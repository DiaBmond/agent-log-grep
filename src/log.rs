use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LogRecord {
    pub line_number: usize,
    pub raw: String,
}

pub fn read_logs(file_path: &str) -> Result<Vec<LogRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut records = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        let record = LogRecord {
            line_number: index + 1,
            raw: line,
        };

        records.push(record);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        env::temp_dir().join(format!(
            "agent_log_grep_{name}_{}_{}.jsonl",
            std::process::id(),
            timestamp
        ))
    }

    #[test]
    fn read_multiple_lines() {
        let path = temp_path("multiple_lines");

        fs::write(&path, "first line\nsecond line\nthird line\n").unwrap();

        let records = read_logs(path.to_str().unwrap()).unwrap();

        assert_eq!(records.len(), 3);

        assert_eq!(records[0].line_number, 1);
        assert_eq!(records[0].raw, "first line");

        assert_eq!(records[1].line_number, 2);
        assert_eq!(records[1].raw, "second line");

        assert_eq!(records[2].line_number, 3);
        assert_eq!(records[2].raw, "third line");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn read_empty_file() {
        let path = temp_path("empty");

        fs::write(&path, "").unwrap();

        let records = read_logs(path.to_str().unwrap()).unwrap();

        assert!(records.is_empty());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn read_nonexistent_file() {
        let path = temp_path("nonexistent");

        let result = read_logs(path.to_str().unwrap());

        assert!(result.is_err());
    }
}
