// https://leetcode.com/problems/maximum-score-of-a-good-subarray/
// 1793. Maximum Score of a Good Subarray
pub struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut left = k;
        let mut right = k;
        let mut max = nums[k];
        let mut min = nums[k];
        while left > 0 || right + 1 < nums.len() {
            if left > 0 && right + 1 < nums.len() {
                if nums[left - 1] > nums[right + 1] {
                    min = min.min(nums[left - 1]);
                    left -= 1;
                } else {
                    min = min.min(nums[right + 1]);
                    right += 1;
                }
            } else if left == 0 {
                min = min.min(nums[right + 1]);
                right += 1;
            } else {
                min = min.min(nums[left - 1]);
                left -= 1;
            }
            max = max.max(min * (right - left + 1) as i32);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_score() {
        assert_eq!(Solution::maximum_score(vec![1, 4, 3, 7, 4, 5], 3), 15);
        assert_eq!(Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
    }
}
