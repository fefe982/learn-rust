// https://leetcode.com/problems/longest-arithmetic-subsequence/
// 1027. Longest Arithmetic Subsequence
pub struct Solution;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::<(usize, i32), i32>::new();
        let mut max = 0;
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                let d = nums[j] - nums[i];
                let cnt = &map.get(&(i, d)).cloned().unwrap_or_default() + 1;
                max = std::cmp::max(max, cnt);
                map.insert((j, d), cnt);
            }
        }
        max + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_arith_seq_length() {
        assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
        assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
        assert_eq!(
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
            4
        );
    }
}
