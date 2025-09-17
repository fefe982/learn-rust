// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
// 1281. Subtract the Product and Sum of Digits of an Integer
pub struct Solution;
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut p = 1;
        let mut s = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            p *= d;
            s += d;
        }
        p - s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subtract_product_and_sum() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}
