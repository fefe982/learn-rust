// https://leetcode.cn/problems/om3reC/
// LCR 108. 单词接龙
pub struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        super::super::lc01::lc_127::Solution::ladder_length(begin_word, end_word, word_list)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn ladder_length() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec_str!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec_str!["hot", "dot", "dog", "lot", "log"]
            ),
            0
        );
    }
}
