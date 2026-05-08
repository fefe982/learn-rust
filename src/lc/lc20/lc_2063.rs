// https://leetcode.com/problems/vowels-of-all-substrings/
// 2063. Vowels of All Substrings
pub struct Solution;
impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let bytes = word.as_bytes();
        let mut ans = 0;
        for i in 0..bytes.len() {
            if bytes[i] == b'a' || bytes[i] == b'e' || bytes[i] == b'i' || bytes[i] == b'o' || bytes[i] == b'u' {
                ans += (i as i64 + 1) * (bytes.len() as i64 - i as i64);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_vowels() {
        assert_eq!(Solution::count_vowels("aba".to_string()), 6);
        assert_eq!(Solution::count_vowels("abc".to_string()), 3);
        assert_eq!(Solution::count_vowels("ltcd".to_string()), 0);
    }
}
