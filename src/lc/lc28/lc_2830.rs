// https://leetcode.com/problems/maximize-the-profit-as-the-salesman/
// 2830. Maximize the Profit as the Salesman
pub struct Solution;
impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        let mut oe = vec![vec![]; n + 1];
        for offer in offers {
            oe[offer[1] as usize].push((offer[0] as usize, offer[2]));
        }
        for i in 1..=n {
            dp[i] = dp[i - 1];
            for &(j, k) in &oe[i - 1] {
                dp[i] = dp[i].max(dp[j] + k);
            }
        }
        dp[n]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximize_the_profit() {
        assert_eq!(
            Solution::maximize_the_profit(5, vec_vec![[0, 0, 1], [0, 2, 2], [1, 3, 2]]),
            3
        );
        assert_eq!(
            Solution::maximize_the_profit(5, vec_vec![[0, 0, 1], [0, 2, 10], [1, 3, 2]]),
            10
        );
    }
}
