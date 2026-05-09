// https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/
// 2068. Check Whether Two Strings are Almost Equivalent
pub struct Solution;
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut count = [0i32; 26];
        for b in word1.as_bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for b in word2.as_bytes() {
            count[(b - b'a') as usize] -= 1;
        }
        for c in count {
            if c.abs() > 3 {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_almost_equivalent() {
        assert_eq!(
            Solution::check_almost_equivalent("aaaa".to_string(), "bccb".to_string()),
            false
        );
        assert_eq!(
            Solution::check_almost_equivalent("abcdeef".to_string(), "abaaacc".to_string()),
            true
        );
        assert_eq!(
            Solution::check_almost_equivalent("cccddabba".to_string(), "babababab".to_string()),
            true
        );
    }
}
