struct Solution;
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.len() <= 2 { return true; }

        let mut decreased = false;
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                if decreased {
                    return false;
                } else {
                    decreased = true;
                }
            }
        }

        !decreased || nums[nums.len() - 1] <= nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::check(vec![3, 4, 5, 1, 2]));
        assert!(Solution::check(vec![3, 4, 5, 1, 3]));
        assert!(Solution::check(vec![3, 4, 5, 3, 3]));
        assert!(Solution::check(vec![3, 4, 5, 2, 3]));
        assert!(!Solution::check(vec![3, 4, 5, 1, 4]));
        assert!(!Solution::check(vec![3, 4, 3, 1, 2]));
        assert!(Solution::check(vec![2, 4]));
    }
}