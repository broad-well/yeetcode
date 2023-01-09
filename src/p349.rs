struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let set1 = nums1.into_iter().collect::<HashSet<_>>();
        let set2 = nums2.into_iter().collect::<HashSet<_>>();
        set1.intersection(&set2).map(|x| *x).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
        assert_eq!(Solution::intersection(vec![1, 2, 2, 1], vec![]), vec![]);
        assert_eq!(Solution::intersection(vec![6, 2, 8, 0, 4], vec![3, 5, 0, 8]), vec![8, 0]);
    }
}
