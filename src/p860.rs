struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut register = [0, 0];
        for payment in bills {
            match payment {
                5 => register[0] += 1,
                10 => {
                    register[1] += 1;
                    if register[0] == 0 {
                        return false;
                    }
                    register[0] -= 1;
                },
                20 => {
                    // Prefer $10 + $5 in change over $5 + $5 + $5 in change
                    if register[1] > 0 && register[0] > 0 {
                        register[1] -= 1;
                        register[0] -= 1;
                    } else if register[0] >= 3 {
                        register[0] -= 3;
                    } else {
                        return false;
                    }
                },
                _ => unreachable!()
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let change = vec![5, 5, 5, 10, 20];
        assert!(Solution::lemonade_change(change));
        let change2 = vec![5, 5, 10, 10, 20];
        assert!(!Solution::lemonade_change(change2));
    }
}