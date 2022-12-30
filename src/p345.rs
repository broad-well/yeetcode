struct Solution;
impl Solution {
    #[inline]
    fn is_vowel(ch: char) -> bool {
        match ch.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false
        }
    }

    pub fn reverse_vowels(mut s: String) -> String {
        let mut start = 0usize;
        let mut end = s.len();
        while start < end {
            end -= 1;
            while end > start && !Solution::is_vowel(s.as_bytes()[end].to_ascii_lowercase().into()) {
                end -= 1;
            }
            while start < end && !Solution::is_vowel(s.as_bytes()[start].to_ascii_lowercase().into()) {
                start += 1;
            }
            if start >= end {
                return s;
            }
            unsafe {
                let bytes = s.as_bytes_mut();
                let byte = bytes[start];
                bytes[start] = bytes[end];
                bytes[end] = byte;
            }
            start += 1;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::reverse_vowels("leetcode".to_owned()), "leotcede");
        assert_eq!(Solution::reverse_vowels("BASEMENT".to_owned()), "BESEMANT");
        assert_eq!(Solution::reverse_vowels("".to_owned()), "");
        assert_eq!(Solution::reverse_vowels("AEII".to_owned()), "IIEA");
    }
}