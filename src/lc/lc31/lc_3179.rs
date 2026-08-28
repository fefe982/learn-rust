// https://leetcode.com/problems/find-the-n-th-value-after-k-seconds/
// 3179. Find the N-th Value After K Seconds
pub struct Solution;
impl Solution {
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let div = |mut x: i64, mut y: i64| -> i64 {
            while x % y != 0 {
                let n = MOD / y + 1;
                x = x * n % MOD;
                y = y * n % MOD;
            }
            x / y
        };
        let nn = (n + k - 1) as i64;
        let mut kk = n as i64 - 1;
        if nn - kk < kk {
            kk = nn - kk;
        }
        let mut ans = 1;
        for i in 1..=kk {
            ans = ans * (nn - i + 1) % MOD;
            ans = div(ans, i);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_after_k_seconds() {
        assert_eq!(Solution::value_after_k_seconds(18, 15), 565722720);
        assert_eq!(Solution::value_after_k_seconds(4, 5), 56);
        assert_eq!(Solution::value_after_k_seconds(5, 3), 35);
    }
}
