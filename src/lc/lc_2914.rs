// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/
// 2914. Minimum Number of Changes to Make Binary String Beautiful
pub struct Solution;
impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s = s.as_bytes();
        let mut cnt = 0;
        for i in 0..s.len() / 2 {
            if s[2 * i] != s[2 * i + 1] {
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_changes() {
        assert_eq!(Solution::min_changes("1001".to_string()), 2);
        assert_eq!(Solution::min_changes("10".to_string()), 1);
        assert_eq!(Solution::min_changes("0000".to_string()), 0);
    }
}
