// https://leetcode.com/problems/longest-harmonious-subsequence/
// 594. Longest Harmonious Subsequence
pub struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;
        let mut last = nums[0];
        let mut last_len = 1;
        let mut seq_len = 0;
        for i in 1..nums.len() {
            if nums[i] == last {
                last_len += 1;
                if seq_len > 0 {
                    seq_len += 1;
                    ans = ans.max(seq_len);
                }
            } else {
                if last + 1 == nums[i] {
                    seq_len = last_len + 1;
                    ans = ans.max(seq_len);
                } else {
                    seq_len = 0;
                }
                last_len = 1;
                last = nums[i];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_lhs() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
        assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
        assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
    }
}
