// https://leetcode.com/problems/remove-zeros-in-decimal-representation/
// 3726. Remove Zeros in Decimal Representation
pub struct Solution;
impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        let mut p = 1;
        let mut v = 0;
        let mut n = n;
        while n > 0 {
            if n % 10 != 0 {
                v += (n % 10) * p;
                p *= 10;
            }
            n /= 10;
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_zeros() {
        assert_eq!(Solution::remove_zeros(1020030), 123);
        assert_eq!(Solution::remove_zeros(1), 1);
    }
}
