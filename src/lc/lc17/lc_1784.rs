// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
// 1784. Check if Binary String Has at Most One Segment of Ones
pub struct Solution;
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut one = true;
        for c in s.chars() {
            if c == '1' {
                if !one {
                    return false;
                }
            } else {
                    one = false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_ones_segment() {
        assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
        assert_eq!(Solution::check_ones_segment("110".to_string()), true);
    }
}
