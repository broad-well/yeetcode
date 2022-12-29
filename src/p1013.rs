struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(mut arr: Vec<i32>) -> bool {
        // prefix sum O(arr.len())
        let mut i = 1usize;
        while i < arr.len() {
            arr[i] += arr[i - 1];
            i += 1;
        }
        let last = *arr.last().unwrap();
        if last % 3 != 0 { return false; }

        arr.into_iter()
            .skip_while(|i| *i != last / 3)
            .skip(1)
            .skip_while(|i| *i != last / 3 * 2)
            .skip(1)
            .skip_while(|i| *i != last).next().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let vec = vec![0,2,1,-6,6,-7,9,1,2,0,1];
        assert!(Solution::can_three_parts_equal_sum(vec));
        let vec2 = vec![0,2,1,-6,6,7,9,-1,2,0,1];
        assert!(!Solution::can_three_parts_equal_sum(vec2));
        let vec3 = vec![3,3,6,5,-2,2,5,1,-9,4];
        assert!(Solution::can_three_parts_equal_sum(vec3));
    }

    #[test]
    fn failed_testcase() {
        let vec = vec![1, -1, 1, -1];
        assert!(!Solution::can_three_parts_equal_sum(vec));
    }
}