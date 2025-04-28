// https://leetcode.com/problems/perfect-number/
// 507. Perfect Number
pub struct Solution;
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num < 4 {
            return false;
        }
        let mut sum = 0;
        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                sum += i;
                if i != num / i {
                    sum += num / i;
                }
            }
        }
        sum + 1 == num
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_perfect_number() {
        assert_eq!(Solution::check_perfect_number(28), true);
        assert_eq!(Solution::check_perfect_number(6), true);
        assert_eq!(Solution::check_perfect_number(1), false);
    }
}
