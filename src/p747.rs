struct Solution;
impl Solution {
    // Single scan challenge (no heap)
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max_index = 0usize;
        let mut second_max_index = 1usize;
        for i in 1..nums.len() {
            if nums[i] > nums[max_index] {
                second_max_index = max_index;
                max_index = i;
            } else if nums[i] > nums[second_max_index] {
                second_max_index = i;
            }
        }
        if nums[max_index] != nums[second_max_index] && nums[max_index] >= 2 * nums[second_max_index] {
            max_index as i32
        } else {
            -1
        }
    }
    // You could also heapify in place and pop two
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::dominant_index(vec![1, 3, 2, 6, 0]), 3);
        assert_eq!(Solution::dominant_index(vec![1, 3, 2, 0]), -1);
        assert_eq!(Solution::dominant_index(vec![0, 0]), -1);
        assert_eq!(Solution::dominant_index(vec![0, 1]), 1);
        assert_eq!(Solution::dominant_index(vec![1, 0]), 0);
    }
}