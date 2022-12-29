struct Solution {

}

impl Solution {
    #[inline]
    fn ordinal(ch: char) -> usize {
        ch as usize - 'a' as usize
    }

    fn frequency_list(word: &str) -> [i32; 26] {
        word.chars().fold([0; 26], |mut array, char| {
            array[Solution::ordinal(char)] += 1;
            array
        })
    }

    fn formable(word: &str, source_freq: &[i32; 26]) -> bool {
        word.chars().try_fold([0; 26], |mut array, ch| {
            let ord = Solution::ordinal(ch);
            array[ord] += 1;
            if array[ord] > source_freq[ord] {
                None
            } else {
                Some(array)
            }
        }).is_some()
    }

    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let char_freq = Solution::frequency_list(&chars);
        words.iter().fold(0i32, |total, word| {
            if Solution::formable(word, &char_freq) {
                total + word.len() as i32
            } else {
                total
            }
        })
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_freq_list() {
        let the_str = "hellorust";
        let freqs = Solution::frequency_list(the_str);
        assert_eq!(freqs[11], 2);
        assert_eq!(freqs[0], 0);
        assert_eq!(freqs[7], 1);
    }

    #[test]
    fn test_example() {
        let the_str = "atach";
        let vec: Vec<String> = vec!["cat".to_owned(), "bt".to_owned(), "hat".to_owned(), "tree".to_owned()];
        let sol = Solution::count_characters(vec, the_str.to_owned());
        assert_eq!(sol, 6);
    }
}