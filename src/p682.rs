struct Solution;
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();
        for op in operations {
            match op.chars().next().unwrap() {
                'C' => { stack.pop().unwrap(); },
                'D' => stack.push(stack.last().unwrap() * 2),
                '+' => stack.push(stack[stack.len() - 2] + stack[stack.len() - 1]),
                _ => stack.push(op.parse().unwrap())
            }
        }
        stack.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::cal_points(vec!["5","2","C","D","+"].into_iter().map(|i| i.to_owned()).collect()), 30);
    }
}