// https://leetcode.com/problems/sum-of-number-and-its-reverse/
// 2443. Sum of Number and Its Reverse
pub struct Solution;
impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        for i in 0..=num {
            let mut n = i;
            let mut rev = 0;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            if i + rev == num {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_number_and_reverse() {
        assert_eq!(Solution::sum_of_number_and_reverse(443), true);
        assert_eq!(Solution::sum_of_number_and_reverse(63), false);
        assert_eq!(Solution::sum_of_number_and_reverse(181), true);
    }
}
