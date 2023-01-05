struct Solution;
impl Solution {
    // Apparently brute force O(n^2) can take less time and less memory...
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut freq = HashMap::<i32, i32>::new();
        // Every time we add to result, we should subtract one (adding the nth num means n-1 new pairs found)
        // instead of doing (nums.len()) subtractions by 1, we consolidate the subtracted amount in the initial value here
        let mut result = -(nums.len() as i32);
        for num in nums {
            result += *freq.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 3, 1]), 3);
        assert_eq!(Solution::num_identical_pairs(vec![]), 0);
    }
}