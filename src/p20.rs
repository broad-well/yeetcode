struct Solution;
impl Solution {
    #[inline]
    fn closing(opening: u8) -> u8 {
        // 40 -> 41
        // 91 -> 93
        // 123 -> 125
        opening + 1 + (opening % 2)
    }

    pub fn is_valid(mut s: String) -> bool {
        unsafe {
            let bytes = s.as_bytes_mut();
            // stack in-place
            let mut stack_top = 0usize;
            for i in 0..bytes.len() {
                match bytes[i] {
                    40 | 91 | 123 => {
                        // push byte onto the in-place stack
                        bytes[stack_top] = bytes[i];
                        stack_top += 1;
                    },
                    _ => if stack_top > 0 && bytes[i] == Solution::closing(bytes[stack_top - 1]) {
                        // pop the opening from the stack
                        stack_top -= 1;
                    } else {
                        return false;
                    }
                }
            }
            stack_top == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::is_valid("()[]{}".to_owned()));
        assert!(Solution::is_valid("(((())))".to_owned()));
        assert!(Solution::is_valid("(([{()(){}}[]][])[]{})".to_owned()));
        assert!(Solution::is_valid("([{[{}{}][]}]())".to_owned()));
        assert!(!Solution::is_valid("((())".to_owned()));
        assert!(!Solution::is_valid("([[)]]".to_owned()));
        assert!(Solution::is_valid("".to_owned()));
        assert!(!Solution::is_valid("(()))".to_owned()));
    }
}