// https://leetcode.com/problems/weighted-word-mapping/
// 3838. Weighted Word Mapping
pub struct Solution;
impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .into_iter()
            .map(|w| {
                let weight: i32 = w
                    .chars()
                    .map(|c| (c as u8 - b'a') as usize)
                    .fold(0, |acc, i| acc + weights[i])
                    % 26;
                (b'a' + 25 - weight as u8) as char
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_map_word_weights() {
        assert_eq!(
            Solution::map_word_weights(
                vec_str!["abcd", "def", "xyz"],
                vec![5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2]
            ),
            "rij".to_string()
        );
        assert_eq!(
            Solution::map_word_weights(
                vec_str!["a", "b", "c"],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            ),
            "yyy".to_string()
        );
    }
}
