// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/
// 1961. Check If String Is a Prefix of Array
pub struct Solution;
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut pos = 0;
        for word in words {
            if s[pos..].starts_with(&word) {
                pos += word.len();
                if pos == s.len() {
                    return true;
                }
            } else {
                return false;
            }
        }
        pos == s.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_prefix_string() {
        assert_eq!(
            Solution::is_prefix_string("iloveleetcode".to_string(), vec_str!["i", "love", "leetcode", "apples"]),
            true
        );
        assert_eq!(
            Solution::is_prefix_string("iloveleetcode".to_string(), vec_str!["apples", "i", "love", "leetcode"]),
            false
        );
    }
}
