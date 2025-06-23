// https://leetcode.com/problems/count-prefixes-of-a-given-string/
// 2255. Count Prefixes of a Given String
pub struct Solution;
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let s = s.as_bytes();
        let mut cnt = 0;
        'w: for word in words {
            let word = word.as_bytes();
            if word.len() > s.len() {
                continue;
            }

            for i in 0..word.len() {
                if word[i] != s[i] {
                    continue 'w;
                }
            }
            cnt += 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_prefixes() {
        assert_eq!(
            Solution::count_prefixes(vec_str!["a", "b", "c", "ab", "bc", "abc"], "abc".to_string()),
            3
        );
        assert_eq!(Solution::count_prefixes(vec_str!["a", "a"], "aa".to_string()), 2);
    }
}
