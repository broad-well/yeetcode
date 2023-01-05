struct Solution;
impl Solution {
    const BACKSPACE: u8 = '#' as u8;

    unsafe fn type_it(s: &mut String) {
        let bytes = s.as_bytes_mut();
        let mut writer = 0usize;
        for reader in 0..bytes.len() {
            match bytes[reader] {
                Solution::BACKSPACE => {
                    if writer > 0 {
                        writer -= 1;
                    }
                },
                ch => {
                    bytes[writer] = ch;
                    writer += 1;
                }
            }
        }
        s.truncate(writer)
    }

    // Challenge: no auxiliary space (all in-place)
    pub fn backspace_compare(mut s: String, mut t: String) -> bool {
        unsafe {
            Solution::type_it(&mut s);
            Solution::type_it(&mut t);
        }
        s == t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::backspace_compare("ab#c".to_owned(), "ad#c".to_owned()));
        assert!(Solution::backspace_compare("ab#####c".to_owned(), "a###d#c".to_owned()));
        assert!(Solution::backspace_compare("a##Ccc#".to_owned(), "Ccccc###".to_owned()));
        assert!(!Solution::backspace_compare("abc".to_owned(), "".to_owned()));
        assert!(Solution::backspace_compare("".to_owned(), "aa###".to_owned()));
    }
}