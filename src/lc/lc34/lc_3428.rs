// https://leetcode.com/problems/maximum-and-minimum-sums-of-at-most-size-k-subsequences/
// 3428. Maximum and Minimum Sums of At Most Size K Subsequences
pub struct Solution;
impl Solution {
    pub fn min_max_sums(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut comb = vec![vec![0; k]; nums.len()];
        comb[0][0] = 1;
        const MOD: i64 = 1000000007;
        for i in 1..nums.len() {
            comb[i][0] = 1;
            for j in 1..k {
                comb[i][j] = (comb[i - 1][j] + comb[i - 1][j - 1]) % MOD;
            }
        }
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() {
            let mut c = 0;
            for r in 0..=(k - 1).min(i) {
                c = (c + comb[i][r]) % MOD;
            }
            ans = (ans + c * (nums[i] + nums[nums.len() - i - 1]) as i64) % MOD;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_max_sums() {
        assert_eq!(Solution::min_max_sums(vec![1, 2, 3], 2), 24);
        assert_eq!(Solution::min_max_sums(vec![5, 0, 6], 1), 22);
        assert_eq!(Solution::min_max_sums(vec![1, 1, 1], 2), 12);
    }
}
