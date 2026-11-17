use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct LogRecord {
    pub line_number: usize,
    pub raw: String,
}

pub fn stream_logs(file_path: &Path) -> io::Result<impl Iterator<Item = io::Result<LogRecord>>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let records = reader.lines().enumerate().map(|(index, line)| {
        line.map(|raw| LogRecord {
            line_number: index + 1,
            raw,
        })
    });

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

        let records = stream_logs(&path)
            .unwrap()
            .collect::<io::Result<Vec<_>>>()
            .unwrap();

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

        let records = stream_logs(&path)
            .unwrap()
            .collect::<io::Result<Vec<_>>>()
            .unwrap();

        assert!(records.is_empty());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn read_nonexistent_file() {
        let path = temp_path("nonexistent");

        let result = stream_logs(&path);

        assert!(result.is_err());
    }
}
