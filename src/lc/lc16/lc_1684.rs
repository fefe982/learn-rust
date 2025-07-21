// https://leetcode.com/problems/count-the-number-of-consistent-strings/
// 1684. Count the Number of Consistent Strings
pub struct Solution;
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_vec = vec![false; 26];
        for c in allowed.as_bytes() {
            allowed_vec[(c - b'a') as usize] = true;
        }
        words.into_iter().fold(0, |ans, word| {
            for c in word.as_bytes() {
                if !allowed_vec[(c - b'a') as usize] {
                    return ans;
                }
            }
            ans + 1
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_consistent_strings() {
        assert_eq!(
            Solution::count_consistent_strings("ab".to_string(), vec_str!["ad", "bd", "aaab", "baa", "badab"]),
            2
        );
        assert_eq!(
            Solution::count_consistent_strings("abc".to_string(), vec_str!["a", "b", "c", "ab", "ac", "bc", "abc"]),
            7
        );
        assert_eq!(
            Solution::count_consistent_strings(
                "cad".to_string(),
                vec_str!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
            ),
            4
        );
    }
}
