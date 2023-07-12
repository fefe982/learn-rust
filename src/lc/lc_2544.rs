// https://leetcode.com/problems/alternating-digit-sum/
// 2544. Alternating Digit Sum
pub struct Solution;
impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        let mut i = 0;
        while n > 0 {
            sum += n % 10 * if i % 2 == 0 { 1 } else { -1 };
            n /= 10;
            i += 1;
        }
        if i % 2 == 1 {
            sum
        } else {
            -sum
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alternate_digit_sum() {
        assert_eq!(Solution::alternate_digit_sum(521), 4);
        assert_eq!(Solution::alternate_digit_sum(111), 1);
        assert_eq!(Solution::alternate_digit_sum(886996), 0);
    }
}
