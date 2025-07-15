// https://leetcode.com/problems/minimum-number-of-valid-strings-to-form-target-ii/
// 3292. Minimum Number of Valid Strings to Form Target II
pub struct Solution;
impl Solution {
    fn lps(s: &[u8]) -> Vec<usize> {
        let mut v = vec![0; s.len()];
        let mut len = 0;
        let mut i = 1;
        while i < s.len() {
            if s[i] == s[len] {
                len += 1;
                v[i] = len;
                i += 1;
            } else if len > 0 {
                len = v[len - 1];
            } else {
                v[i] = 0;
                i += 1;
            }
        }
        v
    }
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let mut tlen = target.len();
        let mut ps = vec![0; tlen + 1];
        for mut word in words {
            let wlen = word.len();
            word.push('#');
            word += &target;
            let lps = Self::lps(word.as_bytes());
            for i in 1..=tlen {
                ps[i] = ps[i].max(lps[wlen + i]);
            }
        }
        let mut res = 0;
        while tlen > 0 && ps[tlen] > 0 {
            res += 1;
            tlen -= ps[tlen];
        }
        if tlen == 0 {
            res
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_valid_strings() {
        assert_eq!(
            Solution::min_valid_strings(vec_str!["abc", "aaaaa", "bcdef"], "aabcdabc".to_string()),
            3
        );
        assert_eq!(
            Solution::min_valid_strings(vec_str!["abababab", "ab"], "ababaababa".to_string()),
            2
        );
        assert_eq!(Solution::min_valid_strings(vec_str!["abcdef"], "xyz".to_string()), -1);
    }
}
