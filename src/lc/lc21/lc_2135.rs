// https://leetcode.com/problems/count-words-obtained-after-adding-a-letter/
// 2135. Count Words Obtained After Adding a Letter
pub struct Solution;
impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let start_masks = start_words.iter().map(|word| Self::mask(word)).collect::<HashSet<_>>();

        target_words
            .iter()
            .filter(|word| {
                let mask = Self::mask(word);
                word.bytes()
                    .any(|letter| start_masks.contains(&(mask ^ (1 << (letter - b'a')))))
            })
            .count() as i32
    }

    fn mask(word: &str) -> i32 {
        word.bytes().fold(0, |mask, letter| mask | (1 << (letter - b'a')))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_word_count() {
        assert_eq!(
            Solution::word_count(vec_str!["ant", "act", "tack"], vec_str!["tack", "act", "acti"]),
            2
        );
        assert_eq!(Solution::word_count(vec_str!["ab", "a"], vec_str!["abc", "abcd"]), 1);
    }
}
