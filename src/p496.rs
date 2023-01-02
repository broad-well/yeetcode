

struct Solution;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::{HashMap, BTreeSet};
        let mut result_mapping: HashMap<i32, i32> = HashMap::new();
        let mut candidates: BTreeSet<i32> = BTreeSet::new();
        for num in nums2.into_iter().rev() {
            candidates.retain(|cand| *cand > num);
            if let Some(answer) = candidates.iter().next() {
                result_mapping.insert(num, *answer);
            }
            candidates.insert(num);
        }
        nums1.into_iter()
            .map(|n| *result_mapping.get(&n).unwrap_or(&-1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let nums2 = vec![9, 0, 10, 5, 14, 19, 12, 16, 1, 7, 8, 17, 11, 2, 18, 15, 3, 4, 13, 6];
        let nums1 = vec![9, 5, 14, 19, 1, 17, 15];
        let result = Solution::next_greater_element(nums1, nums2);
        assert_eq!(result, vec![10, 14, 19, -1, 7, 18, -1]);
    }
}