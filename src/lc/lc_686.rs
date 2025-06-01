// https://leetcode.cn/problems/repeated-string-match/
// 686. Repeated String Match
pub struct Solution;
impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut lps = vec![0; b.len()];
        let mut i = 0;
        for j in 1..b.len() {
            while i > 0 && b[j] != b[i] {
                i = lps[i - 1];
            }
            if b[j] == b[i] {
                i += 1;
                lps[j] = i;
            }
        }
        let mut i = 0;
        for j in 0.. {
            while i > 0 && b[i] != a[j % a.len()] {
                i = lps[i - 1];
            }
            if a[j % a.len()] == b[i] {
                i += 1;
            }
            if i == b.len() {
                return (j / a.len()) as i32 + 1;
            }
            if j >= b.len() + a.len() * 2 {
                return -1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repeated_string_match() {
        assert_eq!(
            Solution::repeated_string_match("abab".to_string(), "bababa".to_string()),
            2
        );
        assert_eq!(Solution::repeated_string_match("abcd".to_string(), "bc".to_string()), 1);
        assert_eq!(
            Solution::repeated_string_match("axaxaya".to_string(), "axaya".to_string()),
            1
        );
        assert_eq!(
            Solution::repeated_string_match("aaac".to_string(), "aac".to_string()),
            1
        );
        assert_eq!(
            Solution::repeated_string_match("abc".to_string(), "wxyz".to_string()),
            -1
        );
        assert_eq!(Solution::repeated_string_match("aa".to_string(), "a".to_string()), 1);
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
            3
        );
        assert_eq!(Solution::repeated_string_match("a".to_string(), "aa".to_string()), 2);
    }
}
