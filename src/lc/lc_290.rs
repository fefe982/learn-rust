// https://leetcode.cn/problems/word-pattern/
// 290. Word Pattern
pub struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map = std::collections::HashMap::new();
        let mut map2 = std::collections::HashMap::new();
        let pattern = pattern.as_bytes();
        let s = s.split_ascii_whitespace().collect::<Vec<_>>();
        if pattern.len() != s.len() {
            return false;
        }
        for (&p, &w) in pattern.iter().zip(s.iter()) {
            if let Some(v) = map.insert(p, w) {
                if v != w {
                    return false;
                }
            }
            if let Some(v) = map2.insert(w, p) {
                if v != p {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_pattern() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
    }
}
