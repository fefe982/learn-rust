// https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/
// 2380. Time Needed to Rearrange a Binary String
pub struct Solution;
impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut c0 = 0;
        let mut res = 0;
        for c in s.chars() {
            if c == '0' {
                c0 += 1;
            } else if c0 > 0 {
                res = c0.max(res + 1);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_seconds_to_remove_occurrences() {
        assert_eq!(Solution::seconds_to_remove_occurrences("001000011".to_string()), 7);
        assert_eq!(Solution::seconds_to_remove_occurrences("0110101".to_string()), 4);
        assert_eq!(Solution::seconds_to_remove_occurrences("11100".to_string()), 0);
    }
}
