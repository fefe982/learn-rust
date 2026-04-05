// https://leetcode.com/problems/sum-of-digits-in-base-k/
// 1837. Sum of Digits in Base K
pub struct Solution;
impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut n = n;
        let mut sum = 0;
        while n > 0 {
            sum += n % k;
            n /= k;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_base() {
        assert_eq!(Solution::sum_base(34, 6), 9);
        assert_eq!(Solution::sum_base(10, 10), 1);
    }
}
