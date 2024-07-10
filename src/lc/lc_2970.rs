// https://leetcode.com/problems/count-the-number-of-incremovable-subarrays-i/
// 2970. Count the Number of Incremovable Subarrays I
pub struct Solution;
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut inc = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                inc += 1;
            } else {
                break;
            }
        }
        if inc == nums.len() - 1 {
            return ((nums.len() * (nums.len() + 1)) / 2) as i32;
        }
        let mut dec = nums.len() - 1;
        for i in (1..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                dec -= 1;
            } else {
                break;
            }
        }
        let mut ans = (nums.len() - dec + 1) as i32;
        for i in 0..=inc {
            while dec < nums.len() && nums[dec] <= nums[i] {
                dec += 1;
            }
            ans += (nums.len() - dec + 1) as i32;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_incremovable_subarray_count() {
        assert_eq!(Solution::incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
        assert_eq!(Solution::incremovable_subarray_count(vec![8, 7, 6, 6]), 3);
    }
}
