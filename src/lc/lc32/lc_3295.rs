// https://leetcode.com/problems/report-spam-message/
// 3295. Report Spam Messages
pub struct Solution;
impl Solution {
    pub fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool {
        let mut s = std::collections::HashSet::new();
        for word in banned_words {
            s.insert(word);
        }
        let mut count = 0;
        for word in message {
            if s.contains(&word) {
                count += 1;
            }
        }
        count >= 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn report_spam() {
        assert_eq!(
            Solution::report_spam(vec_str!["hello", "world", "leetcode"], vec_str!["world", "hello"]),
            true
        );
        assert_eq!(
            Solution::report_spam(
                vec_str!["hello", "programming", "fun"],
                vec_str!["world", "programming", "leetcode"]
            ),
            false
        );
    }
}
