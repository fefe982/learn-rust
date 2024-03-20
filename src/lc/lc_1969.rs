// https://leetcode.com/problems/minimum-non-zero-product-of-the-array-elements/
// 1969. Minimum Non-Zero Product of the Array Elements
pub struct Solution;
const M: i64 = 1_0000_0000_7;
impl Solution {
    fn pow(mut base: i64, mut exp: i64) -> i64 {
        let mut ans = 1;
        while exp > 0 {
            if (exp & 1) == 1 {
                ans = (ans * base) % M;
            }
            base = (base * base) % M;
            exp >>= 1;
        }
        ans
    }
    pub fn min_non_zero_product(p: i32) -> i32 {
        let x = (1i64 << p) - 1;
        let y = (1i64 << (p - 1)) - 1;
        ((Self::pow((x - 1) % M, y) * (x % M)) % M) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_non_zero_product() {
        assert_eq!(Solution::min_non_zero_product(35), 112322054);
        assert_eq!(Solution::min_non_zero_product(1), 1);
        assert_eq!(Solution::min_non_zero_product(2), 6);
        assert_eq!(Solution::min_non_zero_product(3), 1512);
    }
}
