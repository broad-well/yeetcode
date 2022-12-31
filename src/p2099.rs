struct Solution;
use std::collections::{BinaryHeap, HashMap};

struct SortedDrainer {
    heap: BinaryHeap<i32>,
}
impl Iterator for SortedDrainer {
    fn next(&mut self) -> Option<i32> {
        self.heap.pop()
    }
    type Item = i32;
}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let heap = BinaryHeap::from(nums.clone()); // ensure heapify, O(n)
        
        let mut top_freq: HashMap<i32, u32> = SortedDrainer { heap }
            .take(k as usize)
            .fold(HashMap::new(), |mut map, num| {
                map.insert(num, if map.contains_key(&num) {
                    map[&num] + 1
                } else {
                    1
                });
                map
            });
        nums.into_iter()
            .filter(|x| {
                if let Some(freq) = top_freq.get(x) {
                    if *freq > 0 {
                        top_freq.insert(*x, freq - 1);
                        return true;
                    }
                }
                false
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let subseq = Solution::max_subsequence(vec![2, 1, 3, 3], 2);
        assert_eq!(subseq, vec![3, 3]);
        let subseq2 = Solution::max_subsequence(vec![3, 3, 3, 3], 3);
        assert_eq!(subseq2, vec![3, 3, 3]);
        let subseq3 = Solution::max_subsequence(vec![63,-74,61,-17,-55,-59,-10,2,-60,-65], 9);
        assert_eq!(subseq3, vec![63,61,-17,-55,-59,-10,2,-60,-65]);
    }
}