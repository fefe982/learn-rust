// https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
// 1420. Build Array Where You Can Find The Maximum Exactly K Comparisons
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i32) -> Self {
        Self { i: i as i64 }
    }
    fn to_i32(&self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Mul<usize> for IMod {
    type Output = Self;
    fn mul(self, other: usize) -> Self {
        Self {
            i: (self.i * other as i64) % MOD,
        }
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            i: (self.i + other.i) % MOD,
        }
    }
}
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut dp;
        let mut prefix;
        let mut prev_dp = vec![vec![IMod::new(0); (k + 1) as usize]; (m + 1) as usize];
        let mut prev_prefix = prev_dp.clone();
        for j in 1..=m as usize {
            prev_dp[j][1] = IMod::new(1);
            prev_prefix[j][1] = IMod::new(j as i32);
        }
        for _ in 2..=n as usize {
            dp = vec![vec![IMod::new(0); (k + 1) as usize]; (m + 1) as usize];
            prefix = dp.clone();
            for j in 1..=m as usize {
                for k in 1..=k as usize {
                    dp[j][k] = prev_dp[j][k] * j + prev_prefix[j - 1][k - 1];
                    prefix[j][k] = prefix[j - 1][k] + dp[j][k];
                }
            }
            prev_dp = dp;
            prev_prefix = prefix;
        }
        prev_prefix[m as usize][k as usize].to_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_of_arrays() {
        assert_eq!(Solution::num_of_arrays(50, 100, 25), 34549172);
        assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
        assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
        assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
    }
}
