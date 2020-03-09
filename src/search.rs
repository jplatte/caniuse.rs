use crate::data::{
    FeatureData, FEATURES, FEATURE_BIGRAM_INDEX, FEATURE_MONOGRAM_INDEX, FEATURE_TRIGRAM_INDEX,
};

/// Search query contains '`' or a non-ascii character
#[derive(Debug)]
pub struct InvalidSearchQuery;

// TODO: Use tinyvec
pub fn extract_search_terms(query: &str) -> Result<Vec<String>, InvalidSearchQuery> {
    // TODO: Split on non-alphanumeric characters instead?
    query
        .split_whitespace()
        .map(|word| {
            if word.bytes().all(|byte| byte.is_ascii_graphic() && byte != b'`') {
                Ok(word.into())
            } else {
                Err(InvalidSearchQuery)
            }
        })
        .collect()
}

pub fn run_search(
    search_terms: &[String],
    search_scores: &mut Vec<(u16, f64)>,
) -> Vec<FeatureData> {
    for (i, (idx, score)) in search_scores.iter_mut().enumerate() {
        *idx = i as u16;
        *score = 0.0;
    }

    // monogram score: 1
    // bigram score: 4
    // trigram score: 12
    let score_divisor: f64 = search_terms
        .iter()
        .map(|t| {
            (t.as_bytes().windows(3).count() * 12) as f64
                + (t.as_bytes().windows(2).count() * 4) as f64
                + t.as_bytes().len() as f64
        })
        .sum();

    for term in search_terms {
        for monogram in term.as_bytes() {
            if let Some(&feature_indices) = FEATURE_MONOGRAM_INDEX.get(monogram) {
                for &idx in feature_indices {
                    search_scores[idx as usize].1 += 1.0 / score_divisor;
                }
            }
        }

        for bigram in term.as_bytes().windows(2) {
            // &[u8] -> [u8; 2]
            let bigram = match bigram[..] {
                [b1, b2] => [b1, b2],
                _ => unreachable!(),
            };

            if let Some(&feature_indices) = FEATURE_BIGRAM_INDEX.get(&bigram) {
                for &idx in feature_indices {
                    search_scores[idx as usize].1 += 4.0 / score_divisor;
                }
            }
        }

        for trigram in term.as_bytes().windows(3) {
            // &[u8] -> [u8; 3]
            let trigram = match trigram[..] {
                [b1, b2, b3] => [b1, b2, b3],
                _ => unreachable!(),
            };

            if let Some(&feature_indices) = FEATURE_TRIGRAM_INDEX.get(&trigram) {
                for &idx in feature_indices {
                    search_scores[idx as usize].1 += 12.0 / score_divisor;
                }
            }
        }
    }

    search_scores.sort_by(|(idx_a, score_a), (idx_b, score_b)| {
        score_a.partial_cmp(score_b).unwrap().reverse().then_with(|| {
            // Prefer features with shorter titles if scores are equal
            FEATURES[*idx_a as usize].title.len().cmp(&FEATURES[*idx_b as usize].title.len())
        })
    });
    search_scores
        .iter()
        .filter(|(_, score)| *score >= 0.25)
        .map(|(idx, _)| FEATURES[*idx as usize])
        .collect()
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
}
