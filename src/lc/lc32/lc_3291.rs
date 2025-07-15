// https://leetcode.com/problems/minimum-number-of-valid-strings-to-form-target-i/
// 3291. Minimum Number of Valid Strings to Form Target I
pub struct Solution;
impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        super::lc_3292::Solution::min_valid_strings(words, target)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_valid_strings() {
        assert_eq!(
            Solution::min_valid_strings(vec_str!["abc", "aaaaa", "bcdef"], "aabcdabc".to_string()),
            3
        );
        assert_eq!(
            Solution::min_valid_strings(vec_str!["abababab", "ab"], "ababaababa".to_string()),
            2
        );
        assert_eq!(Solution::min_valid_strings(vec_str!["abcdef"], "xyz".to_string()), -1);
    }
}
