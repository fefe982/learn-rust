// https://leetcode.com/problems/longest-uncommon-subsequence-i/
// 521. Longest Uncommon Subsequence I
pub struct Solution;
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_lu_slength() {
        assert_eq!(Solution::find_lu_slength("aba".to_string(), "cdc".to_string()), 3);
        assert_eq!(Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()), 3);
        assert_eq!(Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()), -1);
    }
}
