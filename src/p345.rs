struct Solution;
impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        // collect all vowels and their indices
        let (indices, vowels): (Vec<_>, Vec<_>) = s.char_indices()
            .filter(|(_i, ch)| "aeiou".contains(ch.to_ascii_lowercase()))
            .unzip();
        vowels.into_iter()
            .rev()
            .zip(indices)
            .for_each(|(new_vowel, index)| {
                s.replace_range(index..(index+1), &new_vowel.to_string());
            });
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
    }
}