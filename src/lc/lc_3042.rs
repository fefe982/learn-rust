// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/
// 3042. Count Prefix and Suffix Pairs I
pub struct Solution;
impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        super::lc_3045::Solution::count_prefix_suffix_pairs(words)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_prefix_suffix_pairs() {
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec_str!["a", "aba", "ababa", "aa"]),
            4
        );
        assert_eq!(
            Solution::count_prefix_suffix_pairs(vec_str!["pa", "papa", "ma", "mama"]),
            2
        );
        assert_eq!(Solution::count_prefix_suffix_pairs(vec_str!["abab", "ab"]), 0);
    }
}
