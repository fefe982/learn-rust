// https://leetcode.com/problems/power-of-heroes/
// 2681. Power of Heroes
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
    fn from_i64(i: i64) -> Self {
        Self { i }
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        return IMod::from_i64((self.i + rhs.i) % MOD);
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        self.i = (self.i + rhs.i) % MOD;
    }
}
impl std::ops::Mul<IMod> for IMod {
    type Output = IMod;
    fn mul(self, rhs: IMod) -> Self::Output {
        return IMod::from_i64((self.i * rhs.i) % MOD);
    }
}
impl std::ops::Mul<i32> for IMod {
    type Output = IMod;
    fn mul(self, rhs: i32) -> Self::Output {
        return IMod::from_i64((self.i * rhs as i64) % MOD);
    }
}
impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut res = IMod::new(0);
        let mut sum = res;
        for i in 0..nums.len() {
            let im = IMod::new(nums[i]);
            let ii = im * im;
            res += ii * im;
            res += ii * sum;
            sum = sum * 2 + im;
        }
        res.i as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_power() {
        assert_eq!(Solution::sum_of_power(vec![2, 1, 4]), 141);
        assert_eq!(Solution::sum_of_power(vec![1, 1, 1]), 7);
    }
}
