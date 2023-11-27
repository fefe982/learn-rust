// https://leetcode.com/problems/sum-of-subarray-minimums/
// 907. Sum of Subarray Minimums
pub struct Solution;
const MOD: i64 = 1_000_000_007;
#[derive(Copy, Clone)]
struct IMod {
    val: i64,
}
impl IMod {
    fn new(val: i64) -> Self {
        IMod { val: val % MOD }
    }
    fn to_i32(&self) -> i32 {
        self.val as i32
    }
}
impl std::ops::Add<i64> for IMod {
    type Output = Self;
    fn add(self, other: i64) -> Self {
        IMod {
            val: (self.val + other) % MOD,
        }
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % MOD;
    }
}
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut sum = IMod::new(0);
        let mut stack = vec![(0, 0, IMod::new(0))];
        for (&val, i) in arr.iter().zip(1..) {
            while let Some(&(v, _, _)) = stack.last() {
                if v >= val {
                    stack.pop();
                } else {
                    break;
                }
            }
            let &(_, j, s) = stack.last().unwrap();
            let ns = s + (i - j) as i64 * val as i64;
            sum += ns;
            stack.push((val, i, ns));
        }
        sum.to_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_subarray_mins() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }
}
