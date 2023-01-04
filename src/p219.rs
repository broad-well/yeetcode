struct Solution;
impl Solution {
    #[allow(dead_code, unused)]
    pub fn contains_nearby_duplicate_slow(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;
        if k == 0 { return false; }
        let mut trail = HashSet::<i32>::with_capacity(k as usize * 4);
        let mut i = 0usize;
        while i < nums.len() {
            if trail.contains(&nums[i]) {
                return true;
            } else if trail.len() >= k as usize {
                trail.remove(&nums[i - k as usize]);
            }
            trail.insert(nums[i]);
            i += 1;
        }
        false
    }

    // Apparently this is not much faster
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        if k == 0 { return false; }
        let mut indices = HashMap::<i32, usize>::with_capacity(nums.len() * 2);
        for (i, num) in nums.into_iter().enumerate() {
            match indices.get(&num) {
                Some(j) if i - j <= k as usize => return true,
                _ => { indices.insert(num, i); }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(!Solution::contains_nearby_duplicate(vec![1, 1], 0));
    }
}