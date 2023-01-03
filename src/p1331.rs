struct Solution;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, HashMap};
        let set = arr.iter().copied().collect::<BTreeSet<i32>>();
        let mapping = set.into_iter().enumerate().map(|(i, num)| (num, (i + 1) as i32)).collect::<HashMap<i32, i32>>();
        arr.into_iter().map(|num| mapping[&num]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn samples() {
        let result = Solution::array_rank_transform(vec![100, 400, 200, 300]);
        assert_eq!(result, vec![1, 4, 2, 3]);
    }
}