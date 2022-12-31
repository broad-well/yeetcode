struct Solution;
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let (x1, y1) = (points[1][0] - points[0][0], points[1][1] - points[0][1]);
        let (x2, y2) = (points[2][0] - points[0][0], points[2][1] - points[0][1]);
        // Distinct
        if (x1, y1) == (0, 0) || (x2, y2) == (0, 0) || (x1, y1) == (x2, y2)  {
            return false;
        }
        // Check if <x1, y1> || <x2, y2>
        // Slope approach (handle vertical separately)
        if x1 == 0 && x2 == 0 {
            // On the same vertical line
            return false;
        }
        // Can't touch vertical vectors otherwise
        if x1 == 0 || x2 == 0 {
            // Exactly one of the vectors is vertical
            return true;
        }
        ((y1 as f64) / (x1 as f64) - (y2 as f64) / (x2 as f64)).abs() >= 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn samples() {
        assert!(Solution::is_boomerang(vec![vec![1,1],vec![2,3],vec![3,2]]));
        assert!(!Solution::is_boomerang(vec![vec![1,1],vec![2,2],vec![3,3]]));
        assert!(!Solution::is_boomerang(vec![vec![1,1],vec![2,1],vec![3,1]]));
        assert!(Solution::is_boomerang(vec![vec![1,2],vec![1,-2],vec![1,-3]]));
        assert!(!Solution::is_boomerang(vec![vec![1,2],vec![1,-2],vec![2,-3]]));
        assert!(Solution::is_boomerang(vec![vec![0,0],vec![0,2],vec![2,1]]));
    }
}