// https://leetcode.com/problems/maximum-xor-product/
// 2939. Maximum XOR of Two Numbers in an Array
pub struct Solution;
impl Solution {
    pub fn maximum_xor_product(a: i64, b: i64, n: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut m = (1 << n) / 2;
        while m > 0 {
            if a.min(b) & m == 0 {
                a ^= m;
                b ^= m;
            }
            m >>= 1;
        }
        const MOD: i64 = 1_000_000_007;
        ((a % MOD) * (b % MOD) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_xor_product() {
        assert_eq!(Solution::maximum_xor_product(12, 5, 4), 98);
        assert_eq!(Solution::maximum_xor_product(6, 7, 5), 930);
        assert_eq!(Solution::maximum_xor_product(1, 6, 3), 12);
    }
}
