// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/
// 1780. Check if Number is a Sum of Powers of Three
pub struct Solution;
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_powers_of_three() {
        assert_eq!(Solution::check_powers_of_three(12), true);
        assert_eq!(Solution::check_powers_of_three(91), true);
        assert_eq!(Solution::check_powers_of_three(21), false);
    }
}
