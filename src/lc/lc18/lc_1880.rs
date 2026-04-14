// https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/
// 1880. Check if Word Equals Summation of Two Words
pub struct Solution;
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let word_to_num = |word: String| word.into_bytes().iter().fold(0, |num, &b| num * 10 + (b - b'a') as i32);
        word_to_num(first_word) + word_to_num(second_word) == word_to_num(target_word)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_sum_equal() {
        assert_eq!(
            Solution::is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()),
            true
        );
        assert_eq!(
            Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aab".to_string()),
            false
        );
        assert_eq!(
            Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aaaaa".to_string()),
            true
        );
    }
}
