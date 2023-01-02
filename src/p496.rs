

struct Solution;
impl Solution {
    // premature optimization did not go well?
    // pub fn next_greater_element_btree(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    //     use std::collections::{HashMap, BTreeSet};
    //     let mut result_mapping: HashMap<i32, i32> = HashMap::new();
    //     let mut candidates: BTreeSet<i32> = BTreeSet::new();
    //     for num in nums2.into_iter().rev() {
    //         if let Some(answer) = candidates.range((num + 1)..).next() {
    //             result_mapping.insert(num, *answer);
    //         }
    //         candidates.retain(|cand| *cand > num);
    //         candidates.insert(num);
    //     }
    //     nums1.into_iter()
    //         .map(|n| *result_mapping.get(&n).unwrap_or(&-1))
    //         .collect()
    // }

    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut staging_area: Vec<i32> = Vec::with_capacity(nums1.len()/2);
        let mut results: HashMap<i32, i32> = HashMap::new();
        for n in &nums1 {
            results.insert(*n, -1);
        }
        for n in nums2 {
            while !staging_area.is_empty() && n > *staging_area.last().unwrap() {
                results.insert(staging_area.pop().unwrap(), n);
            }
            if results.contains_key(&n) {
                staging_area.push(n);
            }
        }
        nums1.into_iter().map(|x| results[&x]).collect()
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