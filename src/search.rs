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

        match text.find(term) {
            Some(start) => {
                res.push(Span { start, len: term.len() });
            }
            None => {
                // One of the terms couldn't be found a single time => return no matches
                return Vec::new();
            }
        }
    }

    // First match found for each search term, now find any remaining ones
    for (i, term) in search_terms.iter().enumerate() {
        let term = term.as_ref();

        // Continue searching after the first match
        let mut idx = res[i].end();
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
