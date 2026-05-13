// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
// 2114. Maximum Number of Words Found in Sentences
pub struct Solution;
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .into_iter()
            .map(|s| s.split_whitespace().count() as i32)
            .max()
            .unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_most_words_found() {
        assert_eq!(
            Solution::most_words_found(vec_str![
                "alice and bob love leetcode",
                "i think so too",
                "this is great thanks very much"
            ]),
            6
        );
        assert_eq!(
            Solution::most_words_found(vec_str!["please wait", "continue to fight", "continue to win"]),
            3
        );
    }
}
