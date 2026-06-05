// https://leetcode.com/problems/check-if-number-has-equal-digit-count-and-digit-value/
// 2283. Check if Number Has Equal Digit Count and Digit Value
pub struct Solution;
impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut count = [0; 10];
        for c in num.chars() {
            count[c as usize - '0' as usize] += 1;
        }
        for (i, c) in num.chars().enumerate() {
            if count[i] != c as usize - '0' as usize {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_digit_count() {
        assert_eq!(Solution::digit_count("1210".to_string()), true);
        assert_eq!(Solution::digit_count("030".to_string()), false);
    }
}
