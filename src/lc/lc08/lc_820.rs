// https://leetcode.com/problems/short-encoding-of-words/
// 820. Short Encoding of Words
pub struct Solution;
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut words = words
            .into_iter()
            .map(|w| w.chars().rev().collect::<String>())
            .collect::<Vec<_>>();
        words.sort();
        let mut r = words[0].len() + 1;
        for i in 1..words.len() {
            if words[i].starts_with(&words[i - 1]) {
                r -= words[i - 1].len() + 1;
            }
            r += words[i].len() + 1;
        }
        r as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_length_encoding() {
        assert_eq!(Solution::minimum_length_encoding(vec_str!["time", "me", "bell"]), 10);
        assert_eq!(Solution::minimum_length_encoding(vec_str!["t"]), 2);
    }
}
