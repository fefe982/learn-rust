// https://leetcode.com/problems/largest-number-after-digit-swaps-by-parity/
// 2231. Largest Number After Digit Swaps by Parity
pub struct Solution;
impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut even_digits = Vec::new();
        let mut odd_digits = Vec::new();
        let mut n = num;
        while n > 0 {
            let digit = n % 10;
            if digit % 2 == 0 {
                even_digits.push(digit);
            } else {
                odd_digits.push(digit);
            }
            n /= 10;
        }
        even_digits.sort_unstable();
        odd_digits.sort_unstable();
        let mut even_index = 0;
        let mut odd_index = 0;
        let mut result = 0;
        n = num;
        let mut multiplier = 1;
        while n > 0 {
            let digit = n % 10;
            if digit % 2 == 0 {
                result += even_digits[even_index] * multiplier;
                even_index += 1;
            } else {
                result += odd_digits[odd_index] * multiplier;
                odd_index += 1;
            }
            n /= 10;
            multiplier *= 10;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_integer() {
        assert_eq!(Solution::largest_integer(1234), 3412);
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}
