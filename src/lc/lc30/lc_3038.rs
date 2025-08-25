// https://leetcode.com/problems/maximum-number-of-operations-with-the-same-score-i/
// 3038. Maximum Number of Operations With the Same Score I
pub struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut cnt = 1;
        let s = nums[0] + nums[1];
        for i in 1..nums.len() / 2 {
            if nums[i * 2] + nums[i * 2 + 1] == s {
                cnt += 1;
            } else {
                break;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_operations() {
        assert_eq!(Solution::max_operations(vec![3, 2, 1, 4, 5]), 2);
        assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 1);
    }
}
