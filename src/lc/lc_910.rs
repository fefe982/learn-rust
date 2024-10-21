// https://leetcode.com/problems/smallest-range-ii/
// 910. Smallest Range II
pub struct Solution;
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        if len == 1 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let lmin = (nums[0] + k).min(nums[len - 1] - k);
        let lmax = (nums[0] + k).max(nums[len - 1] - k);
        let mut ans = nums[len - 1] - nums[0];
        for i in 1..len {
            ans = ans.min((nums[i - 1] + k).max(lmax) - (nums[i] - k).min(lmin));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_range_i() {
        assert_eq!(Solution::smallest_range_i(vec![4, 8, 2, 7, 2], 5), 6);
        assert_eq!(Solution::smallest_range_i(vec![7, 8, 8], 5), 1);
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
