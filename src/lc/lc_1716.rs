// https://leetcode.com/problems/calculate-money-in-leetcode-bank/
// 1716. Calculate Money in Leetcode Bank
pub struct Solution;
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let m = n / 7;
        let r = n % 7;
        28 * m + 7 * (m - 1) * m / 2 + (2 * m + r + 1) * r / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_money() {
        assert_eq!(Solution::total_money(4), 10);
        assert_eq!(Solution::total_money(10), 37);
        assert_eq!(Solution::total_money(20), 96);
    }
}
