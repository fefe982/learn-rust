// https://leetcode.com/problems/Uh984O/
// LCP 25. 古董键盘
pub struct Solution;
impl Solution {
    pub fn keyboard(k: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 27]; n + 1];
        dp[0][0] = 1;
        let k = k as usize;
        fn cc(x: usize, y: usize) -> i64 {
            let mut ans = 1;
            for i in 1..=y {
                ans = ans * (x + 1 - i) / i;
            }
            (ans % 1_000_000_007) as i64
        }
        for x in 0..=n {
            for i in 1..=26 {
                for j in 0..=k {
                    if x >= j && dp[x - j][i - 1] > 0 {
                        dp[x][i] = (dp[x][i] + dp[x - j][i - 1] * cc(x, j)) % 1000000007;
                    }
                }
            }
        }
        dp[n][26] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyboard() {
        assert_eq!(Solution::keyboard(1, 1), 26);
        assert_eq!(Solution::keyboard(1, 2), 650);
    }
}
