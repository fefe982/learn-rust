// https://leetcode.com/problems/check-if-a-string-can-break-another-string/
// 1433. Check If a String Can Break Another String
pub struct Solution;
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1: Vec<char> = s1.chars().collect();
        let mut s2: Vec<char> = s2.chars().collect();
        s1.sort();
        s2.sort();
        s1.iter().zip(s2.iter()).all(|(a, b)| a <= b) || s2.iter().zip(s1.iter()).all(|(a, b)| a <= b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_if_can_break() {
        assert_eq!(Solution::check_if_can_break("abc".to_string(), "xya".to_string()), true);
        assert_eq!(
            Solution::check_if_can_break("abe".to_string(), "acd".to_string()),
            false
        );
        assert_eq!(
            Solution::check_if_can_break("leetcodee".to_string(), "interview".to_string()),
            true
        );
    }
}
