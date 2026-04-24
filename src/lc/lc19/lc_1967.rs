// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/
// 1967. Number of Strings That Appear as Substrings in Word
pub struct Solution;
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;
        for pattern in patterns {
            if word.contains(&pattern) {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_of_strings() {
        assert_eq!(
            Solution::num_of_strings(vec_str!["a", "abc", "bc", "d"], "abc".to_string()),
            3
        );
        assert_eq!(
            Solution::num_of_strings(vec_str!["a", "b", "c"], "aaaaabbbbb".to_string()),
            2
        );
        assert_eq!(Solution::num_of_strings(vec_str!["a", "a", "a"], "ab".to_string()), 3);
    }
}
