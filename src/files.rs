use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn collect_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    if root.is_file() {
        return Ok(vec![root.to_path_buf()]);
    }

    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "path is not a file or directory",
        ));
    }

    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();

    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();

            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file()
                && matches!(
                    path.extension().and_then(OsStr::to_str),
                    Some("log" | "jsonl")
                )
            {
                files.push(path);
            }
        }
    }

    files.sort_unstable();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn finds_log_files_recursively() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let root = env::temp_dir().join(format!(
            "agent-log-grep-files-{}-{timestamp}",
            std::process::id()
        ));

        let nested = root.join("2026/11");
        fs::create_dir_all(&nested).unwrap();

        let first = root.join("a.jsonl");
        let second = nested.join("b.jsonl");
        let third = nested.join("c.log");

        fs::write(&first, "first log").unwrap();
        fs::write(&second, "second log").unwrap();
        fs::write(&third, "third log").unwrap();
        fs::write(root.join("ignore.txt"), "not a log").unwrap();

        let files = collect_files(&root).unwrap();

        let mut expected = vec![first, second, third];
        expected.sort();

        assert_eq!(files, expected);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_nonexistent_path() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let root = env::temp_dir().join(format!(
            "agent-log-grep-test-{}-{timestamp}",
            std::process::id()
        ));

        fs::create_dir(&root).unwrap();

        let missing = root.join("missing.jsonl");

        assert!(collect_files(&missing).is_err());

        fs::remove_dir(root).unwrap();
    }

    #[test]
    fn accepts_single_file() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = env::temp_dir().join(format!(
            "agent-log-grep-single-{}-{timestamp}.txt",
            std::process::id()
        ));

        fs::write(&path, "hello").unwrap();

        let files = collect_files(&path).unwrap();

        assert_eq!(files, vec![path.clone()]);

        fs::remove_file(path).unwrap();
    }
}
