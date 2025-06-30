// https://leetcode.com/problems/smallest-divisible-digit-product-i/
// 3345. Smallest Divisible Digit Product
pub struct Solution;
impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for i in 0..10 {
            let nn = n + i;
            let mut j = nn / 10;
            if j == 0 {
                j = 1;
            }
            j *= nn % 10;
            if j % t == 0 {
                return nn;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_number() {
        assert_eq!(Solution::smallest_number(10, 2), 10);
        assert_eq!(Solution::smallest_number(15, 3), 16);
    }
}
