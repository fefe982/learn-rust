// https://leetcode.com/problems/partition-array-to-minimize-xor/
// 3599. Partition Array to Minimize XOR
pub struct Solution;
impl Solution {
    pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] ^= nums[i - 1];
        }
        let mut dp = nums.clone();
        for i in 1..k as usize {
            for j in (i..nums.len()).rev() {
                dp[j] = i32::MAX;
                for k in i..=j {
                    dp[j] = dp[j].min(dp[k - 1].max(nums[j] ^ nums[k - 1]));
                }
            }
        }
        dp[nums.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_xor() {
        assert_eq!(Solution::min_xor(vec![1, 2, 3], 2), 1);
        assert_eq!(Solution::min_xor(vec![2, 3, 3, 2], 3), 2);
        assert_eq!(Solution::min_xor(vec![1, 1, 2, 3, 1], 2), 0);
    }
}
