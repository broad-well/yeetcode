struct Solution;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut seen_once: HashMap<i32, bool> = HashMap::with_capacity(nums.len());
        let mut sum: i32 = 0;

        for num in nums {
            match seen_once.get(&num) {
                Some(true) => {
                    // already seen once
                    sum -= num;
                    seen_once.insert(num, false);
                },
                Some(false) => {},
                None => {
                    sum += num;
                    seen_once.insert(num, true);
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
    }
}