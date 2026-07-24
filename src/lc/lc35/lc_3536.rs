// https://leetcode.com/problems/maximum-product-of-two-digits/
// 3536. Maximum Product of Two Digits
pub struct Solution;
impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut d0 = 0;
        let mut d1 = 0;
        let mut n = n;
        while n > 0 {
            let d = n % 10;
            if d > d0 {
                d1 = d0;
                d0 = d;
            } else if d > d1 {
                d1 = d;
            }
            n /= 10;
        }
        d0 * d1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_product() {
        assert_eq!(Solution::max_product(31), 3);
        assert_eq!(Solution::max_product(22), 4);
        assert_eq!(Solution::max_product(124), 8);
    }
}
