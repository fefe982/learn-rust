// https://leetcode.com/problems/longest-continuous-increasing-subsequence/
// 674. Longest Continuous Increasing Subsequence
pub struct Solution;
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut last = i32::MAX;
        let mut len = 0;
        for n in nums {
            if n > last {
                len += 1;
            } else {
                max = max.max(len);
                len = 1;
            }
            last = n;
        }
        max.max(len)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_length_of_lcis() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }
}
