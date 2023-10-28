// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// 1493. Longest Subarray of 1's After Deleting One Element
pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut last = 0;
        let mut max = 0;
        let mut current = 0;
        let mut nzero = 0;
        for n in nums.into_iter().chain([0].into_iter()) {
            if n == 0 {
                nzero += 1;
                max = max.max(last + current);
                last = current;
                current = 0;
            } else {
                current += 1;
            }
        }
        if nzero == 1 {
            max - 1
        } else {
            max
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
    }
}
