// https://leetcode.com/problems/maximum-number-of-operations-with-the-same-score-ii/
// 3040. Maximum Number of Operations With the Same Score II
pub struct Solution;
impl Solution {
    fn max_op(nums: &Vec<i32>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>, s: i32) -> i32 {
        if i >= j {
            return 0;
        }
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        let mut ans = 0;
        if nums[i] + nums[i + 1] == s {
            ans = ans.max(Self::max_op(nums, i + 2, j, memo, s) + 1);
        }
        if nums[i] + nums[j] == s {
            ans = ans.max(Self::max_op(nums, i + 1, j - 1, memo, s) + 1);
        }
        if nums[j - 1] + nums[j] == s && j >= 2 {
            ans = ans.max(Self::max_op(nums, i, j - 2, memo, s) + 1);
        }
        memo[i][j] = ans;
        ans
    }
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut memo = vec![vec![-1; nums.len()]; nums.len()];
        let mut ans = 0;
        ans = ans.max(Self::max_op(
            &nums,
            0,
            nums.len() - 1,
            &mut memo.clone(),
            nums[0] + nums[1],
        ));
        ans = ans.max(Self::max_op(
            &nums,
            0,
            nums.len() - 1,
            &mut memo.clone(),
            nums[nums.len() - 2] + nums[nums.len() - 1],
        ));
        ans = ans.max(Self::max_op(
            &nums,
            0,
            nums.len() - 1,
            &mut memo,
            nums[0] + nums[nums.len() - 1],
        ));
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_operations() {
        assert_eq!(Solution::max_operations(vec![1, 1, 1, 1, 1, 1]), 3);
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 2, 3, 4]), 3);
        assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 2);
    }
}
