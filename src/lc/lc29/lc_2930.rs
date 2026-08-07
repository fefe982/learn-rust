// https://leetcode.com/problems/number-of-strings-which-can-be-rearranged-to-contain-substring/
// 2930. Number of Strings That Appear as Substrings in Word
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn pow(x: i64, n: i64) -> i64 {
        let mut r = 1;
        let mut xx = x;
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                r = (r * xx) % MOD;
            }
            xx = (xx * xx) % MOD;
            n >>= 1;
        }
        r
    }
    pub fn string_count(n: i32) -> i32 {
        let n = n as i64;
        let r0 = Self::pow(26, n);
        let r1 = Self::pow(25, n - 1) * (75 + n) % MOD;
        let r2 = Self::pow(24, n - 1) * (72 + 2 * n) % MOD;
        let r3 = Self::pow(23, n - 1) * (23 + n) % MOD;
        (((r0 - r1 + r2 - r3) % MOD + MOD) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_count() {
        assert_eq!(Solution::string_count(4), 12);
        assert_eq!(Solution::string_count(10), 83943898);
    }
}
