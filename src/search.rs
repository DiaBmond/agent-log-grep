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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }
