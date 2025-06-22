// https://leetcode.com/problems/count-different-palindromic-subsequences/
// 730. Count Different Palindromic Subsequences
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
#[derive(Debug, Clone, Copy)]
struct IMod {
    val: i64,
}
impl IMod {
    fn from_i32(val: i32) -> Self {
        Self { val: val as i64 }
    }
    fn to_i32(&self) -> i32 {
        self.val as i32
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            val: (self.val + rhs.val) % MOD,
        }
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % MOD;
    }
}
impl std::ops::AddAssign<i32> for IMod {
    fn add_assign(&mut self, rhs: i32) {
        self.val = (self.val + rhs as i64) % MOD;
    }
}
impl std::ops::Sub for IMod {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            val: (self.val + MOD - rhs.val) % MOD,
        }
    }
}
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let mut dp = vec![vec![vec![IMod::from_i32(0); 4]; s.len() + 1]; s.len() + 1];
        let s = s.into_bytes().iter().map(|c| c - b'a').collect::<Vec<_>>();
        for i in 0..s.len() {
            dp[i][i + 1][s[i] as usize] = IMod::from_i32(1);
        }
        for l in 2..=s.len() {
            for i in 0..=s.len() - l {
                let j = i + l;
                let last = s[i + l - 1];
                for c in 0..4 {
                    if last == c as u8 && s[i] == last {
                        let mut sum = IMod::from_i32(2);
                        for cc in 0..4 {
                            sum += dp[i + 1][j - 1][cc];
                        }
                        dp[i][j][c] = sum;
                    } else {
                        dp[i][j][c] = dp[i][j - 1][c] + dp[i + 1][j][c] - dp[i + 1][j - 1][c];
                    }
                }
            }
        }
        dp[0][s.len()]
            .iter()
            .fold(IMod::from_i32(0), |acc, &c| acc + c)
            .to_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_palindromic_subsequences() {
        assert_eq!(Solution::count_palindromic_subsequences("bccb".to_string()), 6);
        assert_eq!(
            Solution::count_palindromic_subsequences(
                "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()
            ),
            104860361
        );
    }
}
