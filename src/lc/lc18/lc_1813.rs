// https://leetcode.com/problems/sentence-similarity-iii/
// 1813. Sentence Similarity III
pub struct Solution;
impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let s1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let s2 = sentence2.split_whitespace().collect::<Vec<_>>();
        let mut left = 0;
        while left < s1.len() && left < s2.len() && s1[left] == s2[left] {
            left += 1;
        }
        let mut right = 0;
        while left + right < s1.len() && left + right < s2.len() && s1[s1.len() - 1 - right] == s2[s2.len() - 1 - right]
        {
            right += 1;
        }
        left + right == s1.len() || left + right == s2.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_are_sentences_similar() {
        assert_eq!(
            Solution::are_sentences_similar("A".to_string(), "a A b A".to_string()),
            true
        );
        assert_eq!(
            Solution::are_sentences_similar("Luky".to_string(), "Lucccky".to_string()),
            false
        );
        assert_eq!(
            Solution::are_sentences_similar("My name is Haley".to_string(), "My Haley".to_string()),
            true
        );
        assert_eq!(
            Solution::are_sentences_similar("of".to_string(), "A lot of words".to_string(),),
            false
        );
        assert_eq!(
            Solution::are_sentences_similar("Eating right now".to_string(), "Eating".to_string()),
            true
        );
    }
}
