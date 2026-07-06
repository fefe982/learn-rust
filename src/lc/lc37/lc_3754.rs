// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i/
// 3754. Concatenate Non-Zero Digits and Multiply by Sum I
pub struct Solution;
impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut sum = 0;
        let mut r = 0;
        let mut n = n;
        let mut p10 = 1;
        while n > 0 {
            let d = (n % 10) as i64;
            if d > 0 {
                sum += d;
                r += d * p10;
                p10 *= 10;
            }
            n /= 10;
        }
        r * sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_and_multiply() {
        assert_eq!(Solution::sum_and_multiply(10203004), 12340);
        assert_eq!(Solution::sum_and_multiply(1000), 1);
    }
}
