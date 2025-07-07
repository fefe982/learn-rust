// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
// 1790. Check if One String Swap Can Make Strings Equal
pub struct Solution;
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut d = usize::MAX;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                if d == usize::MAX {
                    d = i;
                } else if d == usize::MAX - 1 {
                    return false;
                } else if s1[d] != s2[i] || s2[d] != s1[i] {
                    return false;
                } else {
                    d = usize::MAX - 1;
                }
            }
        }
        d == usize::MAX || d == usize::MAX - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn are_almost_equal() {
        assert_eq!(Solution::are_almost_equal("aa".to_string(), "ac".to_string()), false);
        assert_eq!(Solution::are_almost_equal("bank".to_string(), "kanb".to_string()), true);
        assert_eq!(
            Solution::are_almost_equal("attack".to_string(), "defend".to_string()),
            false
        );
        assert_eq!(Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()), true);
    }
}
