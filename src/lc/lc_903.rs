// https://leetcode.com/problems/valid-permutations-for-di-sequence/
// 903. Valid Permutations for DI Sequence
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
#[derive(Clone, Copy)]
struct IMod {
    val: i64,
}
impl IMod {
    fn from_i32(v: i32) -> Self {
        Self { val: v as i64 }
    }
    fn as_i32(&self) -> i32 {
        self.val as i32
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % MOD;
    }
}
impl std::ops::Add for IMod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            val: (self.val + rhs.val) % MOD,
        }
    }
}
impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let mut dp = vec![IMod::from_i32(1)];
        for (i, c) in s.chars().enumerate() {
            let mut ndp = vec![IMod::from_i32(0); i + 2];
            if c == 'D' {
                for j in (0..=i).rev() {
                    ndp[j] = ndp[j + 1] + dp[j];
                }
            } else {
                for j in 1..=i + 1 {
                    ndp[j] = ndp[j - 1] + dp[j - 1];
                }
            }
            dp = ndp;
        }
        dp.into_iter().fold(IMod::from_i32(0), |acc, x| acc + x).as_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_perms_di_sequence() {
        assert_eq!(Solution::num_perms_di_sequence(String::from("DID")), 5);
        assert_eq!(Solution::num_perms_di_sequence(String::from("D")), 1);
    }
}
