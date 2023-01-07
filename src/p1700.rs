struct Solution;
impl Solution {
    pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        // To improve queue pop/push time complexity
        let mut queue = VecDeque::from(students);
        sandwiches.reverse();
        loop {
            let init_sandwich_count = sandwiches.len();
            for _ in 0..queue.len() {
                let student_pref = queue.pop_front().unwrap();
                if *sandwiches.last().unwrap() == student_pref {
                    // preferred, take it and leave the queue
                    sandwiches.pop();
                } else {
                    // not preferred, leave it and re-enter the queue
                    queue.push_back(student_pref);
                }
            }
            if sandwiches.len() == init_sandwich_count {
                // nobody accepted, we are done
                return queue.len() as i32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
        assert_eq!(Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]), 3);
    }
}