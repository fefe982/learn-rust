// https://leetcode.com/problems/largest-merge-of-two-strings/
// 1754. Largest Merge Of Two Strings
pub struct Solution;
impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut word1 = word1.chars().collect::<Vec<_>>();
        let mut word2 = word2.chars().collect::<Vec<_>>();
        let mut res = vec![];
        while !word1.is_empty() && !word2.is_empty() {
            if word1 > word2 {
                res.push(word1.remove(0));
            } else {
                res.push(word2.remove(0));
            }
        }
        res.extend(word1);
        res.extend(word2);
        res.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_merge() {
        assert_eq!(
            Solution::largest_merge("cabaa".to_string(), "bcaaa".to_string()),
            "cbcabaaaaa".to_string()
        );
        assert_eq!(
            Solution::largest_merge("abcabc".to_string(), "abdcaba".to_string()),
            "abdcabcabcaba".to_string()
        );
    }
}
