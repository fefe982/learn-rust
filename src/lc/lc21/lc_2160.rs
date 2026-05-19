// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
// 2160. Minimum Sum of Four Digit Number After Splitting Digits
pub struct Solution;
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = [0; 4];
        let mut num = num;
        for i in 0..4 {
            digits[i] = num % 10;
            num /= 10;
        }
        digits.sort_unstable();
        digits[0] * 10 + digits[1] * 10 + digits[2] + digits[3]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_sum() {
        assert_eq!(Solution::minimum_sum(2932), 52);
        assert_eq!(Solution::minimum_sum(4009), 13);
    }
}
