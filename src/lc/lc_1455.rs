// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/
// 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence
pub struct Solution;
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let sentence = sentence.as_bytes();
        let search_word = search_word.as_bytes();
        let mut i = 1;
        let mut skip = false;
        let mut j = 0;
        for &c in sentence.iter() {
            if c == b' ' {
                skip = false;
                j = 0;
                i += 1;
            } else if !skip {
                if c == search_word[j] {
                    j += 1;
                    if j == search_word.len() {
                        return i;
                    }
                } else {
                    skip = true;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prefix_of_word() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
        assert_eq!(
            Solution::is_prefix_of_word("this problem is an easy problem".to_string(), "pro".to_string()),
            2
        );
        assert_eq!(
            Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()),
            -1
        );
    }
}
