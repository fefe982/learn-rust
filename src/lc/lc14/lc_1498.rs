// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
// 1498. Number of Subsequences That Satisfy the Given Sum Condition
pub struct Solution;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i64) -> Self {
        Self { i: i as i64 }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
    fn pow2(mut p: usize) -> Self {
        let mut p2 = IMod::new(2);
        let mut res = IMod::new(1);
        while p != 0 {
            if p % 2 == 1 {
                res *= p2;
            }
            p /= 2;
            p2 *= p2;
        }
        res
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        return IMod::new((self.i + rhs.i) % 1000000007);
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        self.i = (self.i + rhs.i) % 1000000007;
    }
}
impl std::ops::Mul<IMod> for IMod {
    type Output = IMod;
    fn mul(self, rhs: IMod) -> Self::Output {
        return IMod::new((self.i * rhs.i) % 1000000007);
    }
}
impl std::ops::MulAssign<IMod> for IMod {
    fn mul_assign(&mut self, rhs: IMod) {
        self.i = (self.i * rhs.i) % 1000000007;
    }
}
impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut l = 0;
        let mut r = nums.len() - 1;
        if nums[0] + nums[0] > target {
            return 0;
        }
        let mut cnt = IMod::new(0);
        loop {
            while l <= r && nums[r] + nums[l] > target {
                r -= 1;
            }
            if l > r {
                break;
            }
            cnt += IMod::pow2(r - l);
            l += 1;
        }
        cnt.val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_subseq() {
        assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
        assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
        assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
    }
}
