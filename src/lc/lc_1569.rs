// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/
// 1569. Number of Ways to Reorder Array to Get Same BST
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i64) -> Self {
        Self { i: i as i64 }
    }
    fn val1(self) -> i32 {
        if self.i > 0 {
            self.i as i32 - 1
        } else {
            MOD as i32 - 1
        }
    }
    fn inv(self) -> IMod {
        let mut a = 1;
        let mut b = self.i;
        while b > 1 {
            let m = MOD / b + 1;
            a = a * m % MOD;
            b = b * m % MOD;
        }
        IMod::new(a)
    }
    fn combination(mut a: i32, mut b: i32) -> Self {
        if a - b < b {
            b = a - b;
        }
        let mut n = IMod::new(1);
        let mut d = IMod::new(1);
        while b > 0 {
            n *= a;
            d *= b;
            a -= 1;
            b -= 1;
        }
        n / d
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        return IMod::new((self.i + rhs.i) % MOD);
    }
}
impl std::ops::Mul<IMod> for IMod {
    type Output = IMod;
    fn mul(self, rhs: IMod) -> Self::Output {
        return IMod::new((self.i * rhs.i) % MOD);
    }
}
impl std::ops::MulAssign<IMod> for IMod {
    fn mul_assign(&mut self, rhs: IMod) {
        self.i = (self.i * rhs.i) % MOD;
    }
}
impl std::ops::MulAssign<i32> for IMod {
    fn mul_assign(&mut self, rhs: i32) {
        self.i = (self.i * rhs as i64) % MOD;
    }
}
impl std::ops::Div<IMod> for IMod {
    type Output = IMod;
    fn div(self, rhs: Self) -> IMod {
        if self.i % rhs.i == 0 {
            IMod::new(self.i / rhs.i)
        } else {
            self * rhs.inv()
        }
    }
}
impl Solution {
    fn cnt_way(nums: &Vec<i32>, s: usize, lbound: i32, hbound: i32) -> IMod {
        if s >= nums.len() {
            return IMod::new(1);
        }
        let mut nl = 0;
        let mut nh = 0;
        let mut sl = 0;
        let mut sh = 0;
        for i in s + 1..nums.len() {
            if nums[i] <= lbound || nums[i] >= hbound {
                continue;
            }
            if nums[i] < nums[s] {
                if nl == 0 {
                    sl = i;
                }
                nl += 1;
            } else {
                if nh == 0 {
                    sh = i;
                }
                nh += 1;
            }
        }
        let n = nl + nh;
        let mut res = IMod::new(1);
        if nl > 0 {
            res *= Self::cnt_way(nums, sl, lbound, nums[s]);
        }
        if nh > 0 {
            res *= Self::cnt_way(nums, sh, nums[s], hbound);
        }
        res * IMod::combination(n, nl)
    }
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        Self::cnt_way(&nums, 0, i32::MIN, i32::MAX).val1()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_of_ways() {
        assert_eq!(Solution::num_of_ways(vec![2, 1, 3]), 1);
        assert_eq!(Solution::num_of_ways(vec![3, 4, 5, 1, 2]), 5);
        assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
    }
}
