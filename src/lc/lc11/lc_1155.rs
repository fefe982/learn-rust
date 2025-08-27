// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/
// 1155. Number of Dice Rolls With Target Sum
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    val: i64,
}
impl IMod {
    fn new(val: i32) -> Self {
        Self { val: val as i64 }
    }
    fn val(&self) -> i32 {
        self.val as i32
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, other: Self) {
        self.val = (self.val + other.val) % MOD;
    }
}
impl Solution {
    fn find_num_rolls_to_target(
        n: usize,
        k: usize,
        target: usize,
        cache: &mut Vec<Vec<IMod>>,
    ) -> IMod {
        if target < n || target > n * k {
            return IMod::new(0);
        }
        if n == 1 || target == n * k || target == n {
            return IMod::new(1);
        }
        if cache[n][target].val() != 0 {
            return cache[n][target];
        }
        let mut sum = IMod::new(0);
        for i in 1..=k.min(target - n + 1) {
            sum += Self::find_num_rolls_to_target(n - 1, k, target - i, cache)
        }
        cache[n][target] = sum;
        sum
    }
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        Self::find_num_rolls_to_target(
            n as usize,
            k as usize,
            target as usize,
            &mut vec![vec![IMod::new(0); target as usize + 1]; n as usize + 1],
        )
        .val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_rolls_to_target() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
        assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
    }
}
