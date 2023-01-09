struct Solution;
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut freq_diff = [0i32; 26];
        for ch in word1.as_bytes() {
            freq_diff[(*ch - b'a') as usize] += 1;
        }
        for ch in word2.as_bytes() {
            freq_diff[(*ch - b'a') as usize] -= 1;
        }
        freq_diff.into_iter().find(|diff| diff.abs() > 3).is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::check_almost_equivalent("aabdcda".to_owned(), "dca".to_owned()));
        assert!(!Solution::check_almost_equivalent("aaaa".to_owned(), "dc".to_owned()));
        assert!(Solution::check_almost_equivalent("abcdeef".to_owned(), "abaaacc".to_owned()));
    }
}