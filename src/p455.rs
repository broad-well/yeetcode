// Greedy hypothesis: greedily satisfying the least greedy children leads to the most children satisfied.
struct Solution;
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut gi = 0usize;
        let mut si = 0usize;
        let mut count = 0;
        while gi < g.len() {
            while si < s.len() && g[gi] > s[si] {
                si += 1;
            }
            if si >= s.len() {
                break;
            }
            count += 1;
            gi += 1;
            si += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn random() {
        let g = vec![1, 4, 2, 1, 5, 3];
        let s = vec![5, 3, 2, 2, 2, 1];
        assert_eq!(Solution::find_content_children(g, s), 5);
    }
    #[test]
    fn samples() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        assert_eq!(Solution::find_content_children(g, s), 1);
    }
}