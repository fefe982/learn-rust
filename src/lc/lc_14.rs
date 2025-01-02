// https://leetcode.com/problems/longest-common-prefix/
// 14. Longest Common Prefix
pub struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut len = strs[0].len();
        let f = strs[0].as_bytes();
        for i in 1..strs.len() {
            let s = strs[i].as_bytes();
            let mut j = 0;
            while j < len && j < s.len() && f[j] == s[j] {
                j += 1;
            }
            len = j;
        }
        strs[0][..len].to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["flower", "flow", "flight"]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec_str!["dog", "racecar", "car"]), "");
    }
}
