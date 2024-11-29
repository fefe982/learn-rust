// https://leetcode.com/problems/count-substrings-that-can-be-rearranged-to-contain-a-string-i/
// 3297. Count Substrings That Can Be Rearranged to Contain a String I
pub struct Solution;
impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        super::lc_3298::Solution::valid_substring_count(word1, word2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_substring_count() {
        assert_eq!(
            Solution::valid_substring_count("bcca".to_string(), "bca".to_string()),
            1
        );
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()),
            10
        );
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()),
            0
        );
    }
}
