struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut inserter = 0usize;
        let mut finder = 0usize;
        while finder < nums.len() {
            if nums[finder] != 0 {
                if inserter != finder {
                    nums[inserter] = nums[finder];
                }
                inserter += 1;
            }
            finder += 1;
        }
        while inserter < nums.len() {
            nums[inserter] = 0;
            inserter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let mut nums = vec![0,1,0,3,12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);    
        nums = vec![1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}