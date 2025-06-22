// https://leetcode.com/problems/minimum-size-subarray-sum/
// 209. Minimum Size Subarray Sum
pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min = usize::MAX;
        let mut right = 0;
        let mut sum = 0;
        for left in 0..nums.len() {
            while sum < target && right < nums.len() {
                sum += nums[right];
                right += 1;
            }
            if sum < target {
                break;
            }
            min = min.min(right - left);
            sum -= nums[left];
        }
        if min == usize::MAX {
            0
        } else {
            min as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
