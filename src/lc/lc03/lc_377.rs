// https://leetcode.com/problems/combination-sum-iv/
// 377. Combination Sum IV
pub struct Solution;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &num in &nums {
                if i >= num {
                    dp[i as usize] += dp[(i - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combination_sum4() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }
}
