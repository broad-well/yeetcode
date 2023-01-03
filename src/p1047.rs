struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut chars: Vec<char> = Vec::new();
        for ch in s.chars() {
            match chars.last() {
                Some(last) if *last == ch => { chars.pop(); },
                _ => { chars.push(ch); }
            };
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn samples() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_owned()), "ca");
        assert_eq!(Solution::remove_duplicates("azxxzy".to_owned()), "ay");
        assert_eq!(Solution::remove_duplicates("aaaabbbccdee".to_owned()), "bd");
    }
}