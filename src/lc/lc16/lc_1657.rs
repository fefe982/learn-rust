// https://leetcode.com/problems/determine-if-two-strings-are-close/
// 1657. Determine if Two Strings Are Close
pub struct Solution;
impl Solution {
    fn get_hist(word: &str) -> (Vec<i32>, Vec<i32>) {
        let mut exist = vec![0; 26];
        let mut hash = vec![0; 26];
        for &c in word.as_bytes() {
            hash[(c - b'a') as usize] += 1;
            exist[(c - b'a') as usize] = 1;
        }
        hash.sort_unstable();
        (hash, exist)
    }
    pub fn close_strings(word1: String, word2: String) -> bool {
        word1.len() == word2.len() && Solution::get_hist(&word1) == Solution::get_hist(&word2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_close_strings() {
        assert_eq!(Solution::close_strings("uau".to_string(), "ssx".to_string()), false);
        assert_eq!(Solution::close_strings("abc".to_string(), "bca".to_string()), true);
        assert_eq!(Solution::close_strings("a".to_string(), "aa".to_string()), false);
        assert_eq!(
            Solution::close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        );
    }
}
