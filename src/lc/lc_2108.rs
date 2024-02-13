// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
// 2108. Find First Palindromic String in the Array
pub struct Solution;
impl Solution {
    fn is_panlidromic(word: &[u8]) -> bool {
        for i in 0..word.len() / 2 {
            if word[i] != word[word.len() - 1 - i] {
                return false;
            }
        }
        true
    }
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            if Solution::is_panlidromic(word.as_bytes()) {
                return word;
            }
        }
        "".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_first_palindrome() {
        assert_eq!(
            Solution::first_palindrome(vec_str!["abc", "car", "ada", "racecar", "cool"]),
            "ada"
        );
        assert_eq!(
            Solution::first_palindrome(vec_str!["notapalindrome", "racecar"]),
            "racecar"
        );
        assert_eq!(Solution::first_palindrome(vec_str!["def", "ghi"]), "");
    }
}
