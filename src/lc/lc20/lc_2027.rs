// https://leetcode.com/problems/minimum-moves-to-convert-string/
// 2027. Minimum Moves to Convert String
pub struct Solution;
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut ans = 0;
        while i < s.len() {
            if s[i] == b'X' {
                ans += 1;
                i += 3;
            } else {
                i += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_moves() {
        assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
        assert_eq!(Solution::minimum_moves("XXOX".to_string()), 2);
        assert_eq!(Solution::minimum_moves("OOOO".to_string()), 0);
    }
}
