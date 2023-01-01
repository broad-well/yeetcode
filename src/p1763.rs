struct Solution;


#[inline]
fn togglecase(ch: char) -> char {
    if ch.is_ascii_uppercase() {
        ch.to_ascii_lowercase()
    } else {
        ch.to_ascii_uppercase()
    }
}

impl Solution {

    fn seek_nice_prefix(sub: &[char]) -> (usize, Option<usize>) {
        use std::collections::{HashMap, HashSet};
        // Return (end index of the nice prefix, exclusive) and (earliest possible start index of a potentially longer nice substring inside the given substring)
        let mut unsatisfied: HashMap<char, usize> = HashMap::new();
        let mut satisfied: HashSet<char> = HashSet::new();
        let mut end_index = 0usize;
        for (i, ch) in sub.iter().enumerate() {
            if !satisfied.contains(&ch.to_ascii_lowercase()) {
                if unsatisfied.remove(&togglecase(*ch)).is_some() {
                    satisfied.insert(ch.to_ascii_lowercase());
                } else {
                    unsatisfied.insert(*ch, i);
                    continue;
                }
            }
            if unsatisfied.is_empty() {
                end_index = i;
            }
        }
        let mut unsatisfied_indices: Vec<usize> = unsatisfied.values().copied().collect();
        unsatisfied_indices.push(0);

        unsatisfied_indices.sort_unstable();
        let next_start = unsatisfied_indices.windows(2)
            .find(|pair| pair[1] - pair[0] > 2)
            .map(|pair| pair[0] + 1)
            .or(unsatisfied.into_values().max().map(|i| i + 1));

        (end_index, next_start)
    }

    pub fn longest_nice_substring(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut start_index = 0usize;
        let mut longest: &[char] = &[];
        loop {
            let (end_index, next_start_option) = Solution::seek_nice_prefix(&chars[start_index..]);
            if end_index > 0 && end_index + 1 > longest.len() {
                longest = &chars[start_index..=start_index + end_index];
            }
            if let Some(next_start) = next_start_option {
                start_index += next_start;
            } else {
                break;
            }
        }
        longest.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::longest_nice_substring("YazaAay".to_owned()), "aAa");
        assert_eq!(Solution::longest_nice_substring("AaaaAaBxXXxxxGccDcdC".to_owned()), "AaaaAa");
        assert_eq!(Solution::longest_nice_substring("Bb".to_owned()), "Bb");
    }

    #[test]
    fn failed_testcases() {
        assert_eq!(Solution::longest_nice_substring("FeOZJEnNfjz".to_owned()), "nN");
        assert_eq!(Solution::longest_nice_substring("xLeElzxgHzcWslEdgMGwEOZCXwwDMwcEhgJHLL".to_owned()), "LeEl");
        assert_eq!(Solution::longest_nice_substring("zUXxizubXNKAUGXTjmAXkpzNZMnRBgddDUAWPUa".to_owned()), "ddD");
        assert_eq!(Solution::longest_nice_substring("b".to_owned()), "");
    }
}