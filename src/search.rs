#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

impl Span {
    fn end(self) -> usize {
        self.start + self.len
    }
}

/// Search query contains '`' or a non-ascii character
#[derive(Debug)]
pub struct InvalidSearchQuery;

// TODO: Use tinyvec
pub fn extract_search_terms(query: &str) -> Result<Vec<String>, InvalidSearchQuery> {
    // TODO: Split on non-alphanumeric characters instead?
    query
        .split_whitespace()
        .map(|word| {
            if word.is_ascii() && !word.contains('`') {
                Ok(word.into())
            } else {
                Err(InvalidSearchQuery)
            }
        })
        .collect()
}

pub fn get_text_matches(text: &str, search_terms: &[impl AsRef<str>]) -> Vec<Span> {
    // TODO: fuzzy matching
    let mut res = Vec::new();
    for term in search_terms {
        let term = term.as_ref();

        // Search terms should have been obtained using `extract_search_terms`, which filters out
        // any words containing '`'
        assert!(!term.contains('`'));

        let mut idx = 0;
        while let Some(pos) = text[idx..].find(term) {
            let span = Span { start: idx + pos, len: term.len() };
            idx = span.end();
            res.push(span);
        }
    }

    // Don't use unstable_sort because docs say it's slower for sequences of
    // concatenated sorted lists, which is exactly what we have here.
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_no_search_terms() {
        assert!(extract_search_terms("").unwrap().is_empty());
        assert!(extract_search_terms(" ").unwrap().is_empty());
        assert!(extract_search_terms("  ").unwrap().is_empty());
    }

    #[test]
    fn extract_single_search_term() {
        assert_eq!(extract_search_terms("test").unwrap(), vec!["test".to_owned()]);
        assert_eq!(extract_search_terms(" test   ").unwrap(), vec!["test".to_owned()]);
    }

    #[test]
    fn extract_multiple_search_terms() {
        assert_eq!(extract_search_terms("a b").unwrap(), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(extract_search_terms(" a b ").unwrap(), vec!["a".to_owned(), "b".to_owned()]);
        assert_eq!(extract_search_terms("  a  b  ").unwrap(), vec!["a".to_owned(), "b".to_owned()]);
    }

    #[test]
    fn extract_invalid_search_term() {
        assert!(extract_search_terms("`").is_err());
        assert!(extract_search_terms(" ` ").is_err());
        assert!(extract_search_terms(" `a`").is_err());
        assert!(extract_search_terms(" x `").is_err());
    }

    #[test]
    fn get_no_text_match() {
        assert!(get_text_matches("", &["test"]).is_empty());
    }

    #[test]
    fn get_single_text_match() {
        assert_eq!(get_text_matches("test", &["test"]), vec![Span { start: 0, len: 4 }]);
        assert_eq!(
            get_text_matches("testtest", &["test"]),
            vec![Span { start: 0, len: 4 }, Span { start: 4, len: 4 }]
        );
    }

    #[test]
    fn get_multiple_text_matches() {
        assert_eq!(
            get_text_matches("a b c", &["b", "c"]),
            vec![Span { start: 2, len: 1 }, Span { start: 4, len: 1 }]
        );
        assert_eq!(
            get_text_matches("a b c", &["c", "b"]),
            vec![Span { start: 2, len: 1 }, Span { start: 4, len: 1 }]
        );
    }
}
