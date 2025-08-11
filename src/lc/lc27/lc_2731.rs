// https://leetcode.com/problems/movement-of-robots/
// 2731. Movement of Robots
pub struct Solution;
const MOD: i64 = 10_0000_0007;
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i64) -> Self {
        Self { i }
    }
    fn val(&self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Mul<i64> for IMod {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Self {
            i: self.i * rhs % MOD,
        }
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.i = (self.i + rhs.i) % MOD;
    }
}
impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        let len = nums.len() as i64;
        let mut h = std::collections::BinaryHeap::new();
        let d = d as i64;
        for (i, c) in s.chars().enumerate() {
            if c == 'R' {
                h.push(nums[i] as i64 + d);
            } else {
                h.push(nums[i] as i64 - d);
            }
        }
        let mut sum = IMod::new(0);
        let mut last = h.pop().unwrap();
        let mut i = 0i64;
        while let Some(cur) = h.pop() {
            i += 1i64;
            sum += IMod::new(last - cur) * i * (len - i);
            last = cur;
        }
        sum.val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::sum_distance(vec![-2, 0, 2], "RLL".to_string(), 3),
            8
        );
        assert_eq!(Solution::sum_distance(vec![1, 0], "RL".to_string(), 2), 5);
    }
}
