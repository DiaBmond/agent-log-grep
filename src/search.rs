pub fn matches_query(query: &str, text: &str, ignore_case: bool) -> bool {
    if ignore_case {
        text.to_lowercase().contains(&query.to_lowercase())
    } else {
        text.contains(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_query_case_sensitive() {
        assert!(matches_query("error", "error found", false));
        assert!(!matches_query("error", "ERROR found", false));
    }

    #[test]
    fn matches_query_case_insensitive() {
        assert!(matches_query("error", "ERROR found", true));
        assert!(matches_query("ERROR", "error found", true));
    }

    #[test]
    fn matches_query_when_no_match() {
        assert!(!matches_query("error", "everything is fine", false));
    }
}
