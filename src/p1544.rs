struct Solution;
impl Solution {
    #[inline]
    fn swapcase(ch: u8) -> u8 {
        if ch.is_ascii_lowercase() {
            ch.to_ascii_uppercase()
        } else {
            ch.to_ascii_lowercase()
        }
    }

    pub fn make_good(mut s: String) -> String {
        // Challenge: Do not allocate more space (stack, etc.)
        // Do the operation in-place
        unsafe {
            let bytes = s.as_bytes_mut();
            let mut writer = 1usize;
            let mut reader = 1usize;
            while reader < bytes.len() {
                if writer > 0 && Solution::swapcase(bytes[reader]) == bytes[writer - 1] {
                    // found not great pair, annihilate
                    writer -= 1;
                } else {
                    bytes[writer] = bytes[reader];
                    writer += 1;
                }
                reader += 1;
            }
            s.truncate(writer);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::make_good("abBAcC".to_owned()), "");
        assert_eq!(Solution::make_good("LeEeEEEetCCCcccode".to_owned()), "LEtode");
    }
}