// https://leetcode.com/problems/longest-even-odd-subarray-with-threshold/
// 2760. Longest Even Odd Subarray With Threshold
pub struct Solution;
impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut max_len = 0;
        let mut cur_len = 0;
        let mut cur_mod = 0;
        for n in nums.into_iter().chain(vec![threshold + 1].into_iter()) {
            if n > threshold || n % 2 != cur_mod {
                max_len = max_len.max(cur_len);
                if n <= threshold && n % 2 == 0 {
                    cur_len = 1;
                    cur_mod = 1;
                } else {
                    cur_len = 0;
                    cur_mod = 0;
                }
            } else {
                cur_len += 1;
                cur_mod = 1 - cur_mod;
            }
        }
        max_len
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_alternating_subarray() {
        assert_eq!(Solution::longest_alternating_subarray(vec![4, 10, 3], 10), 2);
        assert_eq!(Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5), 3);
        assert_eq!(Solution::longest_alternating_subarray(vec![1, 2], 2), 1);
        assert_eq!(Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4), 3);
    }
}
