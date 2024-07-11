// https://leetcode.com/problems/count-the-number-of-incremovable-subarrays-ii/
// 2972. Count the Number of Incremovable Subarrays II
pub struct Solution;
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let mut inc = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                inc += 1;
            } else {
                break;
            }
        }
        let n = nums.len() as i64;
        if inc == n - 1 {
            return (n * (n + 1)) / 2;
        }
        let mut dec = n - 1;
        for i in (1..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                dec -= 1;
            } else {
                break;
            }
        }
        let mut ans = n - dec + 1;
        for i in 0..=inc {
            while dec < n && nums[dec as usize] <= nums[i as usize] {
                dec += 1;
            }
            ans += n - dec + 1;
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
