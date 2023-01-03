struct Solution;
const ONE: u8 = '1' as u8;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut rel_score = 0;
        let mut max_rel_score = 0;
        let mut max_synced = true;
        let mut initial_score = if bytes[0] == ONE { 0 } else { 1 };
        for i in 1..bytes.len() - 1 {
            let byte = bytes[i];
            if byte == ONE {
                // Score goes down
                rel_score -= 1;
                max_synced = false;
                // Add to right side of initial score (this byte would've been on the right side)
                initial_score += 1;
            } else {
                // Score goes up
                rel_score += 1;
                if max_synced {
                    max_rel_score = rel_score;
                } else if rel_score == max_rel_score {
                    max_synced = true;
                }
            }
        }
        if *bytes.last().unwrap() == ONE {
            initial_score += 1;
        }
        max_rel_score + initial_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::max_score("011101".to_owned()), 5);
        assert_eq!(Solution::max_score("1111".to_owned()), 3);
        assert_eq!(Solution::max_score("00".to_owned()), 1);
    }
}