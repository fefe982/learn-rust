// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/
// 1662. Check If Two String Arrays are Equivalent
pub struct Solution;
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut i1 = word1.iter().map(|w| w.chars()).flatten();
        let mut i2 = word2.iter().map(|w| w.chars()).flatten();
        loop {
            match (i1.next(), i2.next()) {
                (Some(c1), Some(c2)) => {
                    if c1 != c2 {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_array_strings_are_equal() {
        assert_eq!(
            Solution::array_strings_are_equal(vec_str!["ab", "c"], vec_str!["a", "bc"]),
            true
        );
        assert_eq!(
            Solution::array_strings_are_equal(vec_str!["a", "cb"], vec_str!["ab", "c"]),
            false
        );
        assert_eq!(
            Solution::array_strings_are_equal(vec_str!["abc", "d", "defg"], vec_str!["abcddefg"]),
            true
        );
    }
}
