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
