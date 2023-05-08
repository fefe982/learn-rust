// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/
// 188. Best Time to Buy and Sell Stock IV
pub struct Solution;
impl Solution {
    fn solve(
        i: usize,
        k: usize,
        action: usize,
        prices: &Vec<i32>,
        cache: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if k == 0 || i == prices.len() {
            return 0;
        }
        if cache[i][k - 1][action] >= 0 {
            return cache[i][k - 1][action];
        }
        let val = if action == 0 {
            std::cmp::max(
                Self::solve(i + 1, k, 1, prices, cache) - prices[i],
                Self::solve(i + 1, k, 0, prices, cache),
            )
        } else {
            std::cmp::max(
                Self::solve(i + 1, k - 1, 0, prices, cache) + prices[i],
                Self::solve(i + 1, k, 1, prices, cache),
            )
        };
        cache[i][k - 1][action] = val;
        val
    }
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        Self::solve(
            0,
            k as usize,
            0,
            &prices,
            &mut vec![vec![vec![-1; 2]; k as usize]; prices.len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_profit() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
