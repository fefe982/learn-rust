// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/
// 2419. Longest Subarray With Maximum Bitwise AND
pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_len = 0;
        let mut last = 0;
        let mut this_len = 0;
        for n in nums {
            if n == last {
                if n == max {
                    this_len += 1;
                }
            } else {
                max_len = max_len.max(this_len);
                if n > max {
                    max = n;
                    max_len = 0;
                    this_len = 1;
                } else if n == max {
                    this_len = 1;
                } else {
                    this_len = 0;
                }
            }
            last = n;
        }
        max_len.max(this_len)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 2);
        assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1);
    }
}
