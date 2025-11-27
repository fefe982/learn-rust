// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/
// 2825. Make String a Subsequence Using Cyclic Increments
pub struct Solution;
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let mut j = 0;
        for i in 0..str1.len() {
            if (str2[j] + 26 - str1[i]) % 26 <= 1 {
                j += 1;
            }
            if j == str2.len() {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_make_subsequence() {
        assert_eq!(
            Solution::can_make_subsequence("abc".to_string(), "ad".to_string()),
            true
        );
        assert_eq!(Solution::can_make_subsequence("zc".to_string(), "ad".to_string()), true);
        assert_eq!(Solution::can_make_subsequence("ab".to_string(), "d".to_string()), false);
    }
}
