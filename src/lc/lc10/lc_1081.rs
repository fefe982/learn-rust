// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/
// 1081. Smallest Subsequence of Distinct Characters
pub struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        super::super::lc03::lc_316::Solution::remove_duplicate_letters(s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc".to_string()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb".to_string()
        );
    }
}
