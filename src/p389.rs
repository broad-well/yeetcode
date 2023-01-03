struct Solution;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        use std::collections::HashMap;
        let mut set: HashMap<char, u32> = HashMap::new();
        for ch in s.chars() {
            *set.entry(ch).or_insert(0) += 1;
        }
        for ch in t.chars() {
            match set.get_mut(&ch) {
                Some(elem) if *elem == 0 => return ch,
                None => return ch,
                Some(other) => *other -= 1
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::find_the_difference("aaaaba".to_owned(), "aabaaba".to_owned()), 'b');
        assert_eq!(Solution::find_the_difference("abfaa".to_owned(), "fababa".to_owned()), 'b');
        assert_eq!(Solution::find_the_difference("qczzdd".to_owned(), "dzcqzdc".to_owned()), 'c');
        assert_eq!(Solution::find_the_difference("".to_owned(), "v".to_owned()), 'v');
    }
}