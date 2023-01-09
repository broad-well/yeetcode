struct Solution;
impl Solution {
    fn palindrome(letters: &[u8]) -> bool {
        for i in 0..letters.len()/2 {
            if letters[i] != letters[letters.len() - 1 - i] {
                return false;
            }
        }
        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        if s.len() <= 2 { return true; }
        let mut forward = 0usize;
        let mut backward = s.len() - 1;
        let letters = s.as_bytes();
        while forward < backward {
            if letters[forward] != letters[backward] {
                if letters[forward + 1] == letters[backward] && Self::palindrome(&letters[forward+1..backward+1]) {
                    return true;
                } else if letters[backward - 1] == letters[forward] && Self::palindrome(&letters[forward..backward]) {
                    return true;
                } else {
                    return false;
                }
            }
            forward += 1;
            backward -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::valid_palindrome("aba".to_owned()));
        assert!(Solution::valid_palindrome("abcdeffgedcba".to_owned()));
        assert!(Solution::valid_palindrome("test".to_owned()));
        assert!(Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_owned()));
    }
}
