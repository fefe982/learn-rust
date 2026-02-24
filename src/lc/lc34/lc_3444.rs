// https://leetcode.cn/problems/minimum-increments-for-target-multiples-in-an-array/
// 3444. Minimum Increments for Target Multiples in an Array
pub struct Solution;
impl Solution {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        loop {
            if b == 0 {
                return a;
            }
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
        }
    }
    fn lcm(a: i64, b: i64) -> i64 {
        let g = Self::gcd(a, b);
        a / g * b
    }
    pub fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        let n = target.len();
        let mut lcm = vec![0; 1 << n];
        lcm[0] = 1;
        for i in 1..(1 << n) {
            let sub = i & (i - 1);
            lcm[i] = Self::lcm(lcm[sub], target[i.trailing_zeros() as usize] as i64);
        }
        let mut dp = vec![i64::MAX; 1 << n];
        dp[0] = 0;
        for i in 0..nums.len() {
            let num = nums[i] as i64;
            for j in (1..(1 << n)).rev() {
                let mut sub = j;
                while sub > 0 {
                    let sub_lcm = lcm[sub];
                    let add = (sub_lcm - num % sub_lcm) % sub_lcm;
                    dp[j] = dp[j].min(dp[j ^ sub].saturating_add(add));
                    sub = (sub - 1) & j;
                }
            }
        }
        dp[(1 << n) - 1] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_increments() {
        assert_eq!(Solution::minimum_increments(vec![1, 2, 3], vec![4]), 1);
        assert_eq!(Solution::minimum_increments(vec![8, 4], vec![10, 5]), 2);
        assert_eq!(Solution::minimum_increments(vec![7, 9, 10], vec![7]), 0);
    }
}
