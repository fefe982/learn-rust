// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/
// 1750. Minimum Length of String After Deleting Similar Ends
pub struct Solution;
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j && s[i] == s[j] {
            while i < j && s[i] == s[j] {
                i += 1;
            }
            while i < j && s[i - 1] == s[j] {
                j -= 1;
            }
            if i == j && s[i] == s[i - 1] {
                return 0;
            }
        }
        (j - i + 1) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_length() {
        assert_eq!(Solution::minimum_length("ca".to_owned()), 2);
        assert_eq!(Solution::minimum_length("cabaabac".to_owned()), 0);
        assert_eq!(Solution::minimum_length("aabccabba".to_owned()), 3);
    }
}
