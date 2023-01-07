struct Solution;
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a)); // descending order
        for i in 0..nums.len() - 2 {
            let subsum = nums[i + 1] + nums[i + 2];
            if subsum > nums[i] {
                return nums[i] + subsum;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
        assert_eq!(Solution::largest_perimeter(vec![10, 10, 2, 4, 1]), 24);
        assert_eq!(Solution::largest_perimeter(vec![1, 1, 1, 20]), 3);
        assert_eq!(Solution::largest_perimeter(vec![40, 10, 2, 1]), 0);
    }
}