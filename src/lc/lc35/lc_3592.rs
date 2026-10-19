// https://leetcode.com/problems/inverse-coin-change/
// 3592. Inverse Coin Change
pub struct Solution;
impl Solution {
    pub fn find_coins(num_ways: Vec<i32>) -> Vec<i32> {
        let mut coins = vec![];
        let n = num_ways.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            if dp[i] + 1 == num_ways[i - 1] {
                coins.push(i as i32);
                for j in i..=n {
                    dp[j] += dp[j - i];
                }
            }
            if dp[i] != num_ways[i - 1] {
                return vec![];
            }
        }
        coins
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_coins() {
        assert_eq!(Solution::find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]), vec![2, 4, 6]);
        assert_eq!(Solution::find_coins(vec![1, 2, 2, 3, 4]), vec![1, 2, 5]);
        assert_eq!(Solution::find_coins(vec![1, 2, 3, 4, 15]), vec![]);
    }
}
