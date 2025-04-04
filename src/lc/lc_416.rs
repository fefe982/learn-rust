// https://leetcode.com/problems/partition-equal-subset-sum/
// 416. Partition Equal Subset Sum
pub struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for num in nums {
            let num = num as usize;
            if num > target {
                return false;
            }
            for j in (0..=target - num).rev() {
                if dp[j] {
                    dp[j + num] = true;
                }
            }
            if dp[target] {
                return true;
            }
        }
        dp[target]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_partition() {
        assert_eq!(Solution::can_partition(vec![1, 2, 5]), false);
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }
}
