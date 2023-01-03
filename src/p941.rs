struct Solution;
#[derive(PartialEq, Eq)]
enum State {
    Begin, Climbing, Descending
}
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 { return false; }
        let mut state = State::Begin;
        for pair in arr.windows(2).into_iter() {
            let diff = pair[1] - pair[0];
            match (&state, diff) {
                (State::Descending, d) if d >= 0 => return false,
                (State::Begin, d) => if d <= 0 {
                    return false;
                } else {
                    state = State::Climbing;
                },
                (State::Climbing, d) if d < 0 => state = State::Descending,
                (_, d) => if d == 0 { return false; },
            }
        }
        state == State::Descending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorter_than_3() {
        assert!(!Solution::valid_mountain_array(vec![1, 2]));
        assert!(!Solution::valid_mountain_array(vec![1, 0]));
        assert!(!Solution::valid_mountain_array(vec![2]));
        assert!(!Solution::valid_mountain_array(vec![]));
    }

    #[test]
    fn exactly_3() {
        assert!(Solution::valid_mountain_array(vec![0, 2, 1]));
        assert!(!Solution::valid_mountain_array(vec![2, 2, 1]));
        assert!(!Solution::valid_mountain_array(vec![0, 2, 2]));
        assert!(!Solution::valid_mountain_array(vec![0, 1, 2]));
        assert!(!Solution::valid_mountain_array(vec![2, 1, 0]));
    }

    #[test]
    fn longer() {
        assert!(Solution::valid_mountain_array(vec![0, 1, 4, 3, 2]));
        assert!(!Solution::valid_mountain_array(vec![0, 0, 1, 4, 3, 2]));
        assert!(!Solution::valid_mountain_array(vec![0, 2, 3, 5, 4, 2, 2, 1]));
        assert!(!Solution::valid_mountain_array(vec![5, 7, 8, 10, 50, 49, 24, 30, 10, 5]));
        assert!(!Solution::valid_mountain_array(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 9, 8, 2]));
    }
}