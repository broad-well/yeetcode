struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut max_profit = 0;
        for price in prices[1..].iter() {
            if *price - min > max_profit {
                max_profit = *price - min;
            } else if *price < min {
                min = *price;
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::max_profit(vec![7, 5, 4, 2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 2, 4, 1, 5, 2, 7, 6]), 6);
    }
}