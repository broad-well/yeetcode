
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        use std::mem::swap;
        // c(n) = c(n-1) + c(n-2)
        // just the fibonacci sequence, for which we have an optimal O(1) space solution
        let mut a = 1;
        let mut b = 1;
        for _s in 0..n {
            a += b;
            swap(&mut a, &mut b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence() {
        let seq = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, num) in seq.iter().enumerate() {
            assert_eq!(Solution::climb_stairs(i as i32), *num);
        }
    }
}