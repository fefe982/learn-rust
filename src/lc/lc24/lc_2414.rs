// https://leetcode.com/problems/length-of-the-longest-alphabetical-continuous-substring/
// 2414. Length of the Longest Alphabetical Continuous Substring
pub struct Solution;
impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut max = 1;
        let mut len = 1;
        let s = s.as_bytes();
        for i in 1..s.len() {
            if s[i - 1] + 1 == s[i] {
                len += 1;
                max = max.max(len);
            } else {
                len = 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_continuous_substring() {
        assert_eq!(Solution::longest_continuous_substring("abacaba".to_string()), 2);
        assert_eq!(Solution::longest_continuous_substring("abcde".to_string()), 5);
    }
}
