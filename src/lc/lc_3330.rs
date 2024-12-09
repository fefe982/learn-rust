// https://leetcode.com/problems/find-the-original-typed-string-i/
// 3330. Find the Original Typed String I
pub struct Solution;
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut res = 1;
        let word = word.as_bytes();
        for i in 1..word.len() {
            if word[i] == word[i - 1] {
                res += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_possible_string_count() {
        assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
        assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
        assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
    }
}
