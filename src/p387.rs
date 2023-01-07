struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashMap;
        let mut freq = HashMap::<char, u32>::new();
        for ch in s.chars() {
            freq.entry(ch).and_modify(|freq| *freq += 1).or_insert(1);
        }
        s.chars()
            .enumerate()
            .find(|(_i, ch)| freq[ch] <= 1)
            .map(|(i, _ch)| i as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_owned()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_owned()), 2);
        assert_eq!(Solution::first_uniq_char("basedbasedb".to_owned()), -1);
    }
}