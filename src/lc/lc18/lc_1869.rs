// https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/
// 1869. Longer Contiguous Segments of Ones than Zeros
pub struct Solution;
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max_0 = 0;
        let mut max_1 = 0;
        let mut cnt_0 = 0;
        let mut cnt_1 = 0;
        for c in s.chars() {
            if c == '0' {
                cnt_0 += 1;
                cnt_1 = 0;
            } else {
                cnt_1 += 1;
                cnt_0 = 0;
            }
            max_0 = max_0.max(cnt_0);
            max_1 = max_1.max(cnt_1);
        }
        max_1 > max_0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_zero_ones() {
        assert_eq!(Solution::check_zero_ones("1101".to_string()), true);
        assert_eq!(Solution::check_zero_ones("111000".to_string()), false);
        assert_eq!(Solution::check_zero_ones("110100010".to_string()), false);
    }
}
