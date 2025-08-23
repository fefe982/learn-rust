// https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/
// 1031. Maximum Sum of Two Non-Overlapping Subarrays
pub struct Solution;
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let mut nums = nums;
        for idx in 1..nums.len() {
            nums[idx] += nums[idx - 1];
        }
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut max_sum = nums[first_len + second_len - 1];
        let mut max_sum1 = nums[first_len - 1];
        let mut max_sum2 = nums[second_len - 1];
        for idx in first_len + second_len..nums.len() {
            max_sum1 = std::cmp::max(max_sum1, nums[idx - second_len] - nums[idx - second_len - first_len]);
            max_sum = std::cmp::max(max_sum1 + nums[idx] - nums[idx - second_len], max_sum);
            max_sum2 = std::cmp::max(max_sum2, nums[idx - first_len] - nums[idx - first_len - second_len]);
            max_sum = std::cmp::max(max_sum2 + nums[idx] - nums[idx - first_len], max_sum);
        }
        max_sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum_two_no_overlap() {
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
            20
        );
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
            29
        );
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
            31
        );
    }
}
