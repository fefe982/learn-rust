// https://leetcode.com/problems/buy-two-chocolates/
// 2706. Buy Two Chocolates
pub struct Solution;
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let m = prices.into_iter().fold((i32::MAX, i32::MAX), |(a, b), c| {
            if c < a {
                (c, a)
            } else if c < b {
                (a, c)
            } else {
                (a, b)
            }
        });
        if m.0 + m.1 > money {
            money
        } else {
            money - m.0 - m.1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buy_choco() {
        assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
        assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
    }
}
