// https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k/
// 3578. Count Partitions With Max Min Difference at Most K
pub struct Solution;
impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n + 1];
        let mut sum = vec![0; n + 2];
        let mut s = std::collections::BTreeMap::<i32, i32>::new();
        dp[0] = 1;
        sum[1] = 1;
        const MOD: i64 = 1_000_000_007;
        let mut j = 0;
        for (i, &num) in nums.iter().enumerate() {
            s.entry(num).and_modify(|x| *x += 1).or_insert(1);
            while (s.iter().rev().next().unwrap().0 - s.iter().next().unwrap().0) > k {
                let e = *s.entry(nums[j]).and_modify(|x| *x -= 1).or_default();
                if e == 0 {
                    s.remove(&nums[j]);
                }
                j += 1;
            }
            dp[i + 1] = (sum[i + 1] - sum[j] + MOD) % MOD;
            sum[i + 2] = (sum[i + 1] + dp[i + 1]) % MOD;
        }
        dp[n] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_partitions() {
        assert_eq!(Solution::count_partitions(vec![9, 4, 1, 3, 7], 4), 6);
        assert_eq!(Solution::count_partitions(vec![3, 3, 4], 0), 2);
    }
}
