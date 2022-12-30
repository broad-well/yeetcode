use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // a quick pass to check lengths (because .zip doesn't)
        if s.chars().filter(|x| *x == ' ').count() + 1 != pattern.len() {
            return false;
        }
        s.split_whitespace()
            .zip(pattern.chars())
            .try_fold((HashMap::<&str, char>::new(), [false; 26]), |(mut mapping, mut seen), (word, pat_char)| {
                match mapping.get(word) {
                    // Seen mapping is consistent with the past
                    Some(ch) if *ch == pat_char => Some((mapping, seen)),
                    // Seen mapping is new, and the pattern char has never been seen before
                    None if !seen[pat_char as usize - 96] => {
                        mapping.insert(word, pat_char);
                        seen[pat_char as usize - 96] = true;
                        Some((mapping, seen))
                    },
                    // Seen mapping contradicts the past (same word different pattern_char or vice versa)
                    _ => None
                }
            }).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()));
        assert!(!Solution::word_pattern("abb".to_owned(), "dog cat cat dog".to_owned()));
        assert!(!Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()));
        assert!(!Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()));   
    }

    #[test]
    fn failed_testcase() {
        assert!(!Solution::word_pattern("abba".to_owned(), "dog dog dog dog".to_owned()));
    }
}