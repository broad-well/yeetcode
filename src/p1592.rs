struct Solution;
const SPACE: u8 = ' ' as u8;
impl Solution {
    // Extra constraint: No vector of words allowed
    pub fn reorder_spaces(text: String) -> String {
        // unfortunately, two passes necessary
        let mut space_count = 0usize;
        let mut word_count = 0usize;
        let mut wording = false;
        for byte in text.bytes() {
            if byte == SPACE {
                space_count += 1;
                wording = false;
            } else if !wording {
                wording = true;
                word_count += 1;
            }
        }
        let (gap_size, remainder) = if word_count == 1 {
            (0, space_count)
        } else {
            (space_count / (word_count - 1), space_count % (word_count - 1))
        };
        let gap = " ".repeat(gap_size);
        let mut output = String::with_capacity(text.len());
        wording = false;
        for byte in text.bytes() {
            if byte != SPACE {
                output.push(byte as char);
                wording = true;
            } else if wording {
                word_count -= 1;
                if word_count > 0 {
                    output.push_str(&gap);
                } else {
                    break;
                }
                wording = false;
            }
        }
        output.push_str(&(" ".repeat(remainder)));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::reorder_spaces("  this   is  a sentence ".to_owned()), "this   is   a   sentence");
        assert_eq!(Solution::reorder_spaces("  this   is a sentence ".to_owned()), "this  is  a  sentence  ");
        assert_eq!(Solution::reorder_spaces("this  is  a sentence   ".to_owned()), "this  is  a  sentence  ");
        assert_eq!(Solution::reorder_spaces(" this    is a sentence ".to_owned()), "this  is  a  sentence  ");
        assert_eq!(Solution::reorder_spaces("  t".to_owned()), "t  ");
        assert_eq!(Solution::reorder_spaces("x".to_owned()), "x");
    }
}