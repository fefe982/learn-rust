// https://leetcode.com/problems/check-if-a-string-is-an-acronym-of-words/
// 2828. Check if a String Is an Acronym of Words
pub struct Solution;
impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        if words.len() != s.len() {
            return false;
        }
        for (w, c) in words.iter().zip(s.chars()) {
            if !w.starts_with(c) {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_acronym() {
        assert_eq!(
            Solution::is_acronym(vec_str!["alice", "bob", "charlie"], "abc".to_string()),
            true
        );
        assert_eq!(Solution::is_acronym(vec_str!["an", "apple"], "a".to_string()), false);
        assert_eq!(
            Solution::is_acronym(
                vec_str!["never", "gonna", "give", "up", "on", "you"],
                "ngguoy".to_string()
            ),
            true
        );
    }
}
