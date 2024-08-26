// https://leetcode.com/problems/number-of-great-partitions/
// 2518. Number of Great Partitions
pub struct Solution;
impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; nums.len()];
        let sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        if sum < k as i64 * 2 || nums.len() <= 1 {
            return 0;
        }
        let mut total = 1;
        let m = 1_0000_0000_7;
        let len = nums.len();
        for (i, num) in nums.into_iter().enumerate() {
            total = (total * 2) % m;
            for j in 0..=k {
                if j == 0 {
                    dp[i][j as usize] = 0;
                } else if i == 0 {
                    dp[i][j as usize] = if num < j { 2 } else { 1 };
                } else {
                    dp[i][j as usize] = dp[i - 1][j as usize];
                    if num <= j {
                        dp[i][j as usize] = (dp[i - 1][j as usize] + dp[i - 1][(j - num) as usize]) % m;
                    }
                }
            }
        }
        (total - (dp[len - 1][k as usize] * 2) % m + m) % m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_partitions() {
        assert_eq!(Solution::count_partitions(vec![1, 2, 3, 4], 4), 6);
        assert_eq!(Solution::count_partitions(vec![3, 3, 3], 4), 0);
        assert_eq!(Solution::count_partitions(vec![6, 6], 2), 2);
    }
}
