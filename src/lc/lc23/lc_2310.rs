// https://leetcode.com/problems/sum-of-numbers-with-units-digit-k/
// 2310. Sum of Numbers With Units Digit K
pub struct Solution;
impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        for i in 1..=10 {
            if (i * k) % 10 == num % 10 && i * k <= num {
                return i;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_numbers() {
        assert_eq!(Solution::minimum_numbers(58, 9), 2);
        assert_eq!(Solution::minimum_numbers(37, 2), -1);
        assert_eq!(Solution::minimum_numbers(0, 7), 0);
    }
}
