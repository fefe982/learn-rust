// https://leetcode.com/problems/minimum-lines-to-represent-a-line-chart/
// 2280. Minimum Lines to Represent a Line Chart
pub struct Solution;
impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() <= 1 {
            return 0;
        }
        let mut stock_prices = stock_prices;
        stock_prices.sort();
        let mut ans = 1;
        for i in 2..stock_prices.len() {
            let x1 = stock_prices[i - 2][0] - stock_prices[i - 1][0];
            let y1 = stock_prices[i - 2][1] - stock_prices[i - 1][1];
            let x2 = stock_prices[i - 1][0] - stock_prices[i][0];
            let y2 = stock_prices[i - 1][1] - stock_prices[i][1];
            if x1 as i64 * y2 as i64 != x2 as i64 * y1 as i64 {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_lines() {
        assert_eq!(
            Solution::minimum_lines(vec_vec![[1, 7], [2, 6], [3, 5], [4, 4], [5, 4], [6, 3], [7, 2], [8, 1]]),
            3
        );
        assert_eq!(Solution::minimum_lines(vec_vec![[3, 4], [1, 2], [7, 8], [2, 3]]), 1);
    }
}
