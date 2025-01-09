// https://leetcode.com/problems/counting-words-with-a-given-prefix/
// 2185. Counting Words With a Given Prefix
pub struct Solution;
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|&w| w.starts_with(&pref)).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::prefix_count(vec_str!["pay", "attention", "practice", "attend"], "at".to_string()),
            2
        );
        assert_eq!(
            Solution::prefix_count(vec_str!["leetcode", "win", "loops", "success"], "code".to_string()),
            0
        );
    }
}
