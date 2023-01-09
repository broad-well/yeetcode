struct Solution;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        use std::collections::HashSet;
        let mut trail = HashSet::<(i32, i32)>::new();
        trail.insert((0, 0));
        let mut loc = (0, 0);
        for ch in path.as_bytes() {
            loc = match *ch {
                b'N' => (loc.0, loc.1 + 1),
                b'S' => (loc.0, loc.1 - 1),
                b'E' => (loc.0 + 1, loc.1),
                b'W' => (loc.0 - 1, loc.1),
                _ => unreachable!()
            };
            if !trail.insert(loc) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert!(Solution::is_path_crossing("NEESSENNW".to_owned()));
        assert!(Solution::is_path_crossing("EEEEW".to_owned()));
        assert!(Solution::is_path_crossing("NNEESENNWWWWWSSEE".to_owned()));
        assert!(!Solution::is_path_crossing("".to_owned()));
        assert!(!Solution::is_path_crossing("NENENESSWSWSW".to_owned()));
        assert!(!Solution::is_path_crossing("EESSWWNWW".to_owned()));
    }
}