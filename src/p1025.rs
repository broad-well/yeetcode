struct Solution;
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let mut memo: Vec<bool> = Vec::with_capacity(n as usize);
        // 1 -> Alice loses
        memo.push(false);
        // 2 -> Alice wins
        memo.push(true);

        while memo.len() < n as usize {
            let base_num = memo.len() + 1;
            let winning = (1..(base_num / 2))
                .filter(|x| base_num % x == 0)
                .any(|x| !memo[memo.len() - x]);
            memo.push(winning);
        }

        memo[n as usize - 1]
        // 2: A1, n=1 true
        // 3: A1, B1 n=1 false
        // 4: A1 -> B starts with 3 B1, A1, true
        // 5: A1, B starts with 4, false
        // 6: (choose 1: B starts with 5, true)
        // 7: (choose 1: B starts with 6, false)
        // 8: (choose 1: B starts with 7, true)
        // 9: ()
        // if you can choose 1 or 2, then you can probably win
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