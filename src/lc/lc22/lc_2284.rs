// https://leetcode.com/problems/sender-with-largest-word-count/
// 2284. Sender With Largest Word Count
pub struct Solution;
impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let mut map = std::collections::HashMap::new();
        for (m, s) in messages.into_iter().zip(senders.into_iter()) {
            *map.entry(s).or_insert(0) += m.split_whitespace().count() as i32;
        }
        map.into_iter()
            .fold(("".to_string(), 0), |(max_s, max_c), (s, c)| {
                if c > max_c || (c == max_c && s > max_s) {
                    (s, c)
                } else {
                    (max_s, max_c)
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_largest_word_count() {
        assert_eq!(
            Solution::largest_word_count(
                vec_str!["Hello userTwooo", "Hi userThree", "Wonderful day Alice", "Hi Alice"],
                vec_str!["Alice", "userTwo", "userThree", "Alice"]
            ),
            "Alice".to_string()
        );
        assert_eq!(
            Solution::largest_word_count(
                vec_str!["How is leetcode for everyone", "Leetcode is useful for practice"],
                vec_str!["Bob", "Charlie"]
            ),
            "Charlie".to_string()
        );
    }
}
