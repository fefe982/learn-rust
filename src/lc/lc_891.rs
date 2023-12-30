// https://leetcode.com/problems/sum-of-subsequence-widths/
// 891. Sum of Subsequence Widths
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Clone, Copy, Debug)]
struct IMod {
    v: i64,
}
impl IMod {
    fn from_i32(v: i32) -> Self {
        Self { v: v as i64 }
    }
    fn as_i32(&self) -> i32 {
        self.v as i32
    }
}
impl std::ops::Sub for IMod {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            v: (self.v + MOD - rhs.v) % MOD,
        }
    }
}
impl std::ops::Mul<i32> for IMod {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            v: (self.v * rhs as i64) % MOD,
        }
    }
}
impl std::ops::Add for IMod {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: (self.v + rhs.v) % MOD,
        }
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.v = (self.v + rhs.v) % MOD;
    }
}
impl Solution {
    pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut pow = IMod::from_i32(1);
        let mut sum_max = IMod::from_i32(0);
        for &n in nums.iter() {
            sum_max += pow * n;
            pow += pow;
        }
        let mut sum_min = IMod::from_i32(0);
        pow = IMod::from_i32(1);
        for &n in nums.iter().rev() {
            sum_min += pow * n;
            pow += pow;
        }
        (sum_max - sum_min).as_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_subseq_widths() {
        assert_eq!(Solution::sum_subseq_widths(vec![2, 1, 3]), 6);
        assert_eq!(Solution::sum_subseq_widths(vec![2]), 0);
    }
}
