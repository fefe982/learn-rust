// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/
// 2124. Check if All A's Appears Before All B's
pub struct Solution;
impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut seen_b = false;
        for c in s.chars() {
            if c == 'b' {
                seen_b = true;
            } else if seen_b {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_string() {
        assert_eq!(Solution::check_string("aaabbb".to_string()), true);
        assert_eq!(Solution::check_string("abab".to_string()), false);
        assert_eq!(Solution::check_string("bbb".to_string()), true);
    }
}
