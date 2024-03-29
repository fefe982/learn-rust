// https://leetcode.com/problems/minimum-sum-of-mountain-triplets-i/
// 2908. Minimum Sum of Mountain Triplets I
pub struct Solution;
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut minimums_right = vec![0; nums.len()];
        minimums_right[nums.len() - 1] = i32::MAX;
        for i in (0..nums.len() - 1).rev() {
            minimums_right[i] = minimums_right[i + 1].min(nums[i + 1]);
        }
        let mut min = i32::MAX;
        let mut min_left = i32::MAX;
        for i in 1..nums.len() - 1 {
            min_left = min_left.min(nums[i - 1]);
            if min_left < nums[i] && minimums_right[i] < nums[i] {
                min = min.min(min_left + nums[i] + minimums_right[i]);
            }
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
        assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
