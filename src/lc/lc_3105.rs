// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
// 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
pub struct Solution;
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut inc = 1;
        let mut dec = 1;
        let mut cinc = 1;
        let mut cdec = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                cinc += 1;
            } else {
                inc = inc.max(cinc);
                cinc = 1;
            }
            if nums[i] < nums[i - 1] {
                cdec += 1;
            } else {
                dec = dec.max(cdec);
                cdec = 1;
            }
        }
        inc.max(dec).max(cinc).max(cdec)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_monotonic_subarray() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
