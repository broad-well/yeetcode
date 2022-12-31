struct Solution;
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        // I wanted a shortcut, but the best I got was bit manipulation
        let mut steps: i32 = 0;
        loop {
            steps += num & 1;
            num >>= 1;
            if num > 0 {
                steps += 1;
            } else {
                break;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn samples() {
        assert_eq!(Solution::number_of_steps(0), 0);
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(16), 5);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}