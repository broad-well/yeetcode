struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {

    fn new(mut nums: Vec<i32>) -> Self {
        // compute prefix sum
        let mut i: usize = 1;
        while i < nums.len() {
            nums[i] += nums[i - 1];
            i += 1;
        }
        Self {
            nums
        }
    }
    
    // The naive solution (no prefix sums)
    // fn sum_range_naive(&self, left: i32, right: i32) -> i32 {
    //     self.nums[(left as usize)..(right as usize) + 1].iter().sum()
    // }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.nums[right as usize]
        } else {
            self.nums[right as usize] - self.nums[left as usize - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn samples() {
        let array1 = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(array1.sum_range(0, 2), 1);
        assert_eq!(array1.sum_range(2, 5), -1);
        assert_eq!(array1.sum_range(0, 5), -3);
    }
}