// https://leetcode.com/problems/ways-to-express-an-integer-as-sum-of-powers/
// 2787. Ways to Express an Integer as Sum of Powers
pub struct Solution;
impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            let v = i.pow(x as u32) as usize;
            if v > n {
                break;
            }
            for j in (v..=n).rev() {
                dp[j] = (dp[j] + dp[j - v]) % 1000000007;
            }
        }
        dp[n] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_ways() {
        assert_eq!(Solution::number_of_ways(175, 1), 102614114);
        assert_eq!(Solution::number_of_ways(10, 2), 1);
        assert_eq!(Solution::number_of_ways(4, 1), 2);
    }
}
