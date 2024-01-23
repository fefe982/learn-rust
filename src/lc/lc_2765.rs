// https://leetcode.com/problems/longest-alternating-subarray/
// 2765. Longest Alternating Subarray
pub struct Solution;
impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut last = 0;
        let mut len = -1;
        for i in 1..nums.len() {
            let d = nums[i] - nums[i - 1];
            if d.abs() == 1 {
                if len > 0 && d == -last {
                    len += 1;
                } else if d == 1 {
                    len = 2;
                } else {
                    len = -1;
                }
            } else {
                len = -1;
            }
            max = max.max(len);
            last = d;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_alternating_subarray() {
        assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![3, 3, 3, 2, 3, 3, 2, 1, 1]), 2);
        assert_eq!(
            Solution::alternating_subarray(vec![14, 30, 29, 49, 3, 23, 44, 21, 26, 52]),
            -1
        );
        assert_eq!(Solution::alternating_subarray(vec![21, 9, 5]), -1);
        assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
        assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
    }
}
