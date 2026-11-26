// https://leetcode.com/problems/absolute-difference-between-maximum-and-minimum-k-elements/
// 3774. Absolute Difference Between Maximum and Minimum K Elements
pub struct Solution;
impl Solution {
    pub fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut r = 0;
        for i in 0..k as usize {
            r = r + nums[nums.len() - i - 1] - nums[i];
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abs_difference() {
        assert_eq!(Solution::abs_difference(vec![5, 2, 2, 4], 2), 5);
        assert_eq!(Solution::abs_difference(vec![100], 1), 0);
    }
}
