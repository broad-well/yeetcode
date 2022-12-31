struct Solution;
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::divisor_game(100));
    }
}