// https://leetcode.com/problems/count-common-words-with-one-occurrence/
// 2085. Count Common Words With One Occurrence
pub struct Solution;
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut invalid = std::collections::HashSet::new();
        let mut valid1 = std::collections::HashSet::new();
        let mut valid = std::collections::HashSet::new();
        for s in words1 {
            if invalid.contains(&s) {
                continue;
            }
            if valid1.contains(&s) {
                valid1.remove(&s);
                invalid.insert(s);
            } else {
                valid1.insert(s);
            }
        }
        for s in words2 {
            if invalid.contains(&s) {
                continue;
            }
            if valid.contains(&s) {
                valid.remove(&s);
                invalid.insert(s);
            } else if valid1.contains(&s) {
                valid.insert(s);
            }
        }
        valid.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_words() {
        assert_eq!(
            Solution::count_words(
                vec_str!["leetcode", "is", "amazing", "as", "is"],
                vec_str!["amazing", "leetcode", "is"]
            ),
            2
        );
        assert_eq!(
            Solution::count_words(vec_str!["b", "bb", "bbb"], vec_str!["a", "aa", "aaa"]),
            0
        );
        assert_eq!(
            Solution::count_words(vec_str!["a", "ab"], vec_str!["a", "a", "a", "ab"]),
            1
        );
    }
}
