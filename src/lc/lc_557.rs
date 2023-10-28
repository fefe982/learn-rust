// https://leetcode.com/problems/reverse-words-in-a-string-iii/
// 557. Reverse Words in a String III
pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }
}
