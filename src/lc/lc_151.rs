// https://leetcode.com/problems/reverse-words-in-a-string/
// 151. Reverse Words in a String
pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().fold(String::new(), |mut acc, x| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(x);
            acc
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }
}
